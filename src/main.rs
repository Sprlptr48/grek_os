
#![no_std]
#![no_main]


static HELLO: &[u8] = b"How arr u?";

#[no_mangle]
pub extern "C" fn _start()  -> !{

let vga_buffer = 0xb8000 as *mut u8;
 
 for(a,&byte)in HELLO.iter().enumerate(){
   unsafe{
     *vga_buffer.offset(a as isize *2) = byte;
     *vga_buffer.offset(a as isize *2 +1) = 0xd;
   }
  
 }
 
	loop{} //isletim sistemi icin baslangic noktasi
}


use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
	loop{}  // panik durumunda cagrilir
}
