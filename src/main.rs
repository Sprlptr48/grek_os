
#![no_std]
#![no_main]


#[no_mangle]
pub extern "C" fn _start()  -> !{
	loop{} //iþletim sistemi için baþlangýç noktasý
}


use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
	loop{}  // panik durumunda çaðrýlýr
}
