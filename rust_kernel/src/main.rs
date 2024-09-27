#![no_std]
#![no_main]
extern crate alloc;

use bootloader_api::{entry_point, BootInfo};
use log::{error, info};
use rust_kernel::driver::display::frame_buffer::{Color, FRAME_BUFFER_WRITER};
use rust_kernel::{hlt_loop, CONFIG};

entry_point!(kernel_main, config = &CONFIG);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    rust_kernel::init(boot_info);

    loop {
        info!("
[1] Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque feugiat quam at sollicitudin malesuada. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; Etiam felis magna, faucibus at velit vitae, iaculis viverra dui. Cras neque nisl, gravida id velit id, efficitur fermentum libero. Cras eget tempus tellus. Mauris pulvinar blandit turpis sed sodales. Phasellus justo augue, sagittis ac ipsum a, facilisis vestibulum nulla.
[2] Suspendisse cursus, mauris vitae bibendum fermentum, nibh mi accumsan ante, sed pellentesque augue ante ut sem. Sed commodo viverra magna in mollis. Phasellus non posuere ligula, vitae porttitor ex. Nullam porta velit et quam ultrices fermentum. Pellentesque a diam nibh. Suspendisse sit amet interdum lacus, sagittis maximus quam. Donec lacinia, magna quis mattis tempus, dui dui tempus urna, eu vestibulum ipsum elit eget dui. Phasellus ac convallis nunc, quis molestie est.
[3] Sed diam magna, pulvinar non magna porttitor, facilisis elementum felis. Etiam ac libero ut felis blandit interdum. Phasellus ut odio ac est ultricies vestibulum id sed libero. Duis eget libero mauris. Maecenas dignissim nisi nibh. Donec et quam pharetra, hendrerit lectus vel, malesuada ligula. Sed dictum nisl nec tortor auctor, in rutrum libero lobortis. Etiam ac finibus dolor. Cras laoreet sagittis nisi, sit amet rutrum purus pellentesque in.
[4] Aliquam cursus diam a nunc feugiat tempus. Curabitur ultricies, tellus vel fermentum ultrices, ligula tellus dictum nunc, in luctus nisl ipsum a eros. Suspendisse elementum facilisis lectus sit amet egestas. Curabitur ultricies dolor lacinia est pulvinar viverra. Curabitur mollis enim vel mauris commodo convallis. Integer tristique accumsan tortor, non blandit magna aliquet nec. Morbi eu nibh sit amet ex auctor posuere eget sed elit. Quisque porta magna sem, egestas facilisis velit rutrum aliquet. Donec sit amet porta nulla, a faucibus lorem.
[5] Nam eget sem magna. Morbi pharetra neque velit, ut cursus libero varius pulvinar. Sed pellentesque tincidunt odio ac consequat. Praesent ac ultrices velit. Ut augue arcu, semper vel consectetur id, pulvinar in arcu. Nullam dapibus lobortis nibh ac venenatis. Donec hendrerit elit ante, a varius lacus facilisis sit amet. Proin sodales, arcu quis placerat imperdiet, erat ligula finibus lacus, eget porttitor nisi metus luctus augue.
[6] Integer imperdiet maximus laoreet. Interdum et malesuada fames ac ante ipsum primis in faucibus. Donec iaculis sodales felis nec tempus. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; Integer porta porta massa, et tempus diam porttitor quis. Aenean pellentesque est eu hendrerit commodo. Maecenas ut sem non mauris bibendum tempor porta quis libero. Etiam tortor tortor, pulvinar tempor turpis ornare, dapibus suscipit justo.
[7] Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec a eros placerat felis faucibus rhoncus at eu ipsum. Pellentesque in dignissim leo, eget condimentum dui. Aenean accumsan placerat lorem ut efficitur. In quis dolor sed magna vehicula viverra. Fusce iaculis dui et feugiat feugiat. Duis id rhoncus quam. Fusce suscipit ligula orci, sed aliquam nisi tempor non. Quisque sit amet eleifend mauris. Aenean et risus nec est hendrerit rutrum. Nunc malesuada fermentum blandit. Nullam enim erat, vehicula in mollis a, imperdiet finibus est. Vestibulum sodales rhoncus est, a tincidunt velit maximus eget. Proin eu dui viverra, venenatis leo sed, cursus sapien. Praesent convallis lectus quis sapien finibus rutrum.
[8] Aliquam felis felis, aliquam vel vulputate vitae, tincidunt at felis. Ut ultrices purus non lacinia auctor. Sed interdum, leo id egestas sollicitudin, justo metus imperdiet leo, vel ultrices sapien leo vitae metus. Ut ante tortor, fringilla ut accumsan sed, lacinia sed ante. Integer quis consectetur est. Curabitur vel leo vel eros ullamcorper dapibus ac eget orci. Morbi fermentum vehicula tristique. Integer sit amet velit non sem imperdiet semper. Sed faucibus fermentum dolor vel egestas. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; Cras sit amet convallis tellus. Fusce pharetra turpis et neque aliquam sodales. Curabitur aliquet, purus sed imperdiet pulvinar, dui sem rhoncus leo, a porta lectus tellus eget ipsum.
[9] Aliquam erat volutpat. Nunc et felis molestie, rutrum sapien eu, pulvinar ligula. Praesent varius vulputate arcu. Interdum et malesuada fames ac ante ipsum primis in faucibus. Aliquam pulvinar nisl sapien, non euismod diam tincidunt eget. Cras ut vehicula arcu. Sed vestibulum, erat et ullamcorper varius, tellus sapien mattis nulla, id mattis justo metus sit amet mi. Ut sapien lectus, bibendum vel porta at, luctus id metus. Quisque facilisis sapien id justo consequat, a ullamcorper massa placerat. Nullam ac risus auctor, auctor lectus a, euismod nisi. Proin a nisi sapien. Proin leo diam, eleifend in erat ac, congue sodales diam. In dapibus quis eros et volutpat. Quisque aliquam risus nec eros dictum, nec commodo velit gravida.
");
        FRAME_BUFFER_WRITER.lock().clear_screen()
    }

    let mut frame_buffer_writer = FRAME_BUFFER_WRITER.lock();
    frame_buffer_writer.rainbow();
    // for _ in 0..100 {
    //     frame_buffer_writer.fill_screen(Color::new(255,0,0));
    //     frame_buffer_writer.fill_screen(Color::new(0,255,0));
    //     frame_buffer_writer.fill_screen(Color::new(0,0,255));
    // }

    //exit_qemu(Success);
    hlt_loop()
}
