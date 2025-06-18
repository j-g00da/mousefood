macro_rules! for_all_rgb_colors {
    (
        $inner:ident
    ) => {
        $inner!(Rgb555);
        $inner!(Bgr555);
        $inner!(Rgb565);
        $inner!(Bgr565);
        $inner!(Rgb666);
        $inner!(Bgr666);
        $inner!(Rgb888);
        $inner!(Bgr888);
    };
}

pub(crate) use for_all_rgb_colors;
