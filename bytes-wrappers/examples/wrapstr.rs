use lipsum::lipsum;

use bytes_wrappers::{
    wrapper::{BaseTransformer, CRC32Unwrapper, CRC32Wrapper, GammaTransformer, SwapTransformer},
    Transformer,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let text = lipsum(10);
    println!("Source message: {text}");

    let mut wrapper = GammaTransformer::new(
        SwapTransformer::new(CRC32Wrapper::new(BaseTransformer::default())),
        47u64,
    );
    let wrapped = wrapper.transform(text.as_bytes())?;

    println!("Transformed message: {wrapped:X?}");

    let mut unwrapper = CRC32Unwrapper::new(SwapTransformer::new(GammaTransformer::new(
        BaseTransformer::default(),
        47u64,
    )));
    let unwrapped = unwrapper.transform(wrapped)?;

    println!(
        "Recovered message: {}",
        String::from_utf8(unwrapped.to_vec())?
    );

    Ok(())
}
