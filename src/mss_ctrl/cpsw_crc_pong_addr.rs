#[doc = "Register `CPSW_CRC_PONG_ADDR` reader"]
pub type R = crate::R<CpswCrcPongAddrSpec>;
#[doc = "Register `CPSW_CRC_PONG_ADDR` writer"]
pub type W = crate::W<CpswCrcPongAddrSpec>;
#[doc = "Field `undefined` reader - 15:0\\]
This denotes the last address of the data Read. This depends on the Packet size . Example : If packet size is 3072 bytes or 3KB and CRC is disabled , this needs to programmed as 3064 or hBF8 If packet size is 3072 bytes or 3KB and 16 bit CRC , this needs to be programmed as hC00 Address must be 4 byte aligned for 32 bit CRC Address must be 2 byte aligned for 16 bit CRC"]
pub type UndefinedR = crate::FieldReader<u16>;
#[doc = "Field `undefined` writer - 15:0\\]
This denotes the last address of the data Read. This depends on the Packet size . Example : If packet size is 3072 bytes or 3KB and CRC is disabled , this needs to programmed as 3064 or hBF8 If packet size is 3072 bytes or 3KB and 16 bit CRC , this needs to be programmed as hC00 Address must be 4 byte aligned for 32 bit CRC Address must be 2 byte aligned for 16 bit CRC"]
pub type UndefinedW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
This denotes the last address of the data Read. This depends on the Packet size . Example : If packet size is 3072 bytes or 3KB and CRC is disabled , this needs to programmed as 3064 or hBF8 If packet size is 3072 bytes or 3KB and 16 bit CRC , this needs to be programmed as hC00 Address must be 4 byte aligned for 32 bit CRC Address must be 2 byte aligned for 16 bit CRC"]
    #[inline(always)]
    pub fn undefined(&self) -> UndefinedR {
        UndefinedR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
This denotes the last address of the data Read. This depends on the Packet size . Example : If packet size is 3072 bytes or 3KB and CRC is disabled , this needs to programmed as 3064 or hBF8 If packet size is 3072 bytes or 3KB and 16 bit CRC , this needs to be programmed as hC00 Address must be 4 byte aligned for 32 bit CRC Address must be 2 byte aligned for 16 bit CRC"]
    #[inline(always)]
    #[must_use]
    pub fn undefined(&mut self) -> UndefinedW<CpswCrcPongAddrSpec> {
        UndefinedW::new(self, 0)
    }
}
#[doc = "CPSW_CRC_PONG_ADDR\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_crc_pong_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_crc_pong_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswCrcPongAddrSpec;
impl crate::RegisterSpec for CpswCrcPongAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_crc_pong_addr::R`](R) reader structure"]
impl crate::Readable for CpswCrcPongAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_crc_pong_addr::W`](W) writer structure"]
impl crate::Writable for CpswCrcPongAddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_CRC_PONG_ADDR to value 0"]
impl crate::Resettable for CpswCrcPongAddrSpec {
    const RESET_VALUE: u32 = 0;
}
