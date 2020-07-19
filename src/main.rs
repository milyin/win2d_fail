winrt::import!(
    dependencies
        os
        nuget: Win2D.uwp
    types
	microsoft::graphics::canvas::CanvasDevice
);

fn main() -> winrt::Result<()> {
    use microsoft::graphics::canvas::CanvasDevice;
    let _device = CanvasDevice::get_shared_device()?;
    Ok(())
}
