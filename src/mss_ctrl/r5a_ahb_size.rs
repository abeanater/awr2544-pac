#[doc = "Register `R5A_AHB_SIZE` reader"]
pub type R = crate::R<R5aAhbSizeSpec>;
#[doc = "Register `R5A_AHB_SIZE` writer"]
pub type W = crate::W<R5aAhbSizeSpec>;
#[doc = "Field `ahb_size` reader - 4:0\\]
Ti internal Register. Modifying this register is not recommended Code for selecting size for ahb. b00011 4KB b00100 8KB b00101 16KB b00110 32KB b00111 64KB b01000 128KB b01001 256KB b01010 512KB b01011 1MB b01100 2MB b01101 4MB b01110 8MB b01111 16MB b10000 32MB b10001 64MB b10010 128MB b10011 256MB b10100 512MB b10101 1GB b10110 2GB b10111 4GB"]
pub type AhbSizeR = crate::FieldReader;
#[doc = "Field `ahb_size` writer - 4:0\\]
Ti internal Register. Modifying this register is not recommended Code for selecting size for ahb. b00011 4KB b00100 8KB b00101 16KB b00110 32KB b00111 64KB b01000 128KB b01001 256KB b01010 512KB b01011 1MB b01100 2MB b01101 4MB b01110 8MB b01111 16MB b10000 32MB b10001 64MB b10010 128MB b10011 256MB b10100 512MB b10101 1GB b10110 2GB b10111 4GB"]
pub type AhbSizeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Ti internal Register. Modifying this register is not recommended Code for selecting size for ahb. b00011 4KB b00100 8KB b00101 16KB b00110 32KB b00111 64KB b01000 128KB b01001 256KB b01010 512KB b01011 1MB b01100 2MB b01101 4MB b01110 8MB b01111 16MB b10000 32MB b10001 64MB b10010 128MB b10011 256MB b10100 512MB b10101 1GB b10110 2GB b10111 4GB"]
    #[inline(always)]
    pub fn ahb_size(&self) -> AhbSizeR {
        AhbSizeR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Ti internal Register. Modifying this register is not recommended Code for selecting size for ahb. b00011 4KB b00100 8KB b00101 16KB b00110 32KB b00111 64KB b01000 128KB b01001 256KB b01010 512KB b01011 1MB b01100 2MB b01101 4MB b01110 8MB b01111 16MB b10000 32MB b10001 64MB b10010 128MB b10011 256MB b10100 512MB b10101 1GB b10110 2GB b10111 4GB"]
    #[inline(always)]
    #[must_use]
    pub fn ahb_size(&mut self) -> AhbSizeW<R5aAhbSizeSpec> {
        AhbSizeW::new(self, 0)
    }
}
#[doc = "R5A_AHB_SIZE\n\nYou can [`read`](crate::Reg::read) this register and get [`r5a_ahb_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r5a_ahb_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R5aAhbSizeSpec;
impl crate::RegisterSpec for R5aAhbSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r5a_ahb_size::R`](R) reader structure"]
impl crate::Readable for R5aAhbSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`r5a_ahb_size::W`](W) writer structure"]
impl crate::Writable for R5aAhbSizeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets R5A_AHB_SIZE to value 0"]
impl crate::Resettable for R5aAhbSizeSpec {
    const RESET_VALUE: u32 = 0;
}
