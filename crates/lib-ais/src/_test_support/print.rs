use crate::GenResStream;
use futures::StreamExt;
use tokio::io::AsyncWriteExt;

pub async fn print_gen_stream(mut stream: GenResStream) -> Result<(), Box<dyn std::error::Error>> {
	let mut stdout = tokio::io::stdout();
	let mut char_count = 0;

	// let mut final_data_responses = Vec::new();

	while let Some(res) = stream.next().await {
		// NOTE: For now, we just flatten this result list since it will most likely be a vec of one.
		//       However, if res.length > 1, we might want to split the output, as those might be for different responses.
		let res_list = res?;

		for res in res_list {
			let bytes = res.response.as_bytes();

			// Poor man's wrapping
			char_count += bytes.len();
			if char_count > 80 {
				stdout.write_all(b"\n").await?;
				char_count = 0;
			}

			// Write output
			stdout.write_all(bytes).await?;
			stdout.flush().await?;

			// if let Some(final_data) = res.final_data {
			// 	stdout.write_all(b"\n").await?;
			// 	stdout.flush().await?;
			// 	final_data_responses.push(final_data);
			// 	break;
			// }
		}
	}

	stdout.write_all(b"\n").await?;
	stdout.flush().await?;

	Ok(())
}
