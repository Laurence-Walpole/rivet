use anyhow::{Context, Result};
use std::{convert::Infallible, env, net::SocketAddr, process::Command};
use tokio::{
	io::{AsyncReadExt, AsyncWriteExt},
	net::{TcpListener, UdpSocket},
};

#[tokio::main]
async fn main() -> Result<()> {
	// Env
	let envs: Vec<(String, String)> = env::vars().collect();
	println!("Env:\n{:#?}\n", envs);

	// resolv.conf
	let output = Command::new("cat")
		.arg("/etc/resolv.conf")
		.output()
		.expect("Failed to execute command");
	println!(
		"resolv.conf:\n{}\n",
		String::from_utf8_lossy(&output.stdout)
	);

	// Echo servers
	if let Ok(http_port) = env::var("PORT_test_http") {
		let http_port: u16 = http_port.parse()?;
		tokio::spawn(echo_http_server(http_port));
	}

	if let Ok(tcp_port) = env::var("PORT_test_tcp") {
		let tcp_port: u16 = tcp_port.parse()?;
		tokio::spawn(echo_tcp_server(tcp_port));
	}

	if let Ok(udp_port) = env::var("PORT_test_udp") {
		let udp_port: u16 = udp_port.parse()?;
		tokio::spawn(echo_udp_server(udp_port));
	}

	// Lobby ready
	lobby_ready().await?;

	// Wait indefinitely
	println!("Waiting indefinitely...");
	std::future::pending::<()>().await;

	Ok(())
}

async fn lobby_ready() -> Result<()> {
	let url = format!(
		"{}/matchmaker/lobbies/ready",
		env::var("RIVET_API_ENDPOINT").context("RIVET_API_ENDPOINT")?
	);
	let token = env::var("RIVET_TOKEN").context("RIVET_TOKEN")?;

	let client = reqwest::Client::new();
	client
		.post(&url)
		.header("Content-Type", "application/json")
		.header("Authorization", format!("Bearer {}", token))
		.send()
		.await?;

	println!("Success, waiting indefinitely");
	Ok(())
}

async fn echo_http_server(port: u16) {
	use hyper::service::{make_service_fn, service_fn};
	use hyper::{Body, Request, Response, Server};

	let addr = SocketAddr::from(([0, 0, 0, 0], port));
	println!("HTTP: {}", port);

	async fn echo(req: Request<Body>) -> Result<Response<Body>, Infallible> {
		Ok(Response::new(req.into_body()))
	}

	let make_service = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(echo)) });
	Server::bind(&addr)
		.serve(make_service)
		.await
		.expect("hyper server");
}

async fn echo_tcp_server(port: u16) {
	let addr = format!("0.0.0.0:{}", port);
	let listener = TcpListener::bind(&addr).await.expect("bind failed");
	println!("TCP: {}", port);

	loop {
		let (mut socket, _) = listener.accept().await.expect("accept failed");
		tokio::spawn(async move {
			let mut buf = [0u8; 1024];
			loop {
				let n = socket.read(&mut buf).await.expect("read failed");
				if n == 0 {
					break;
				}
				socket.write_all(&buf[0..n]).await.expect("write failed");
			}
		});
	}
}

async fn echo_udp_server(port: u16) -> Result<()> {
	let addr = format!("0.0.0.0:{}", port);
	let socket = UdpSocket::bind(&addr).await?;
	println!("UDP: {}", port);

	let mut buf = vec![0u8; 1024];
	loop {
		let (size, src) = socket.recv_from(&mut buf).await?;
		let data = String::from_utf8_lossy(&buf[..size]);
		println!("Received data: {}", data);

		socket.send_to(&buf[..size], &src).await?;
	}
}
