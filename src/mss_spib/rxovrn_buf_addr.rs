#[doc = "Register `RXOVRN_BUF_ADDR` reader"]
pub type R = crate::R<RxovrnBufAddrSpec>;
#[doc = "Register `RXOVRN_BUF_ADDR` writer"]
pub type W = crate::W<RxovrnBufAddrSpec>;
#[doc = "Field `RXOVRN_BUF_ADDR` reader - 10:0\\]
Address of the RAM location of RXRAM for which an Overwrite occured. This address value will show only the offset address of the RAM location in the Multibuffer RAM address space. Refer to the device Spec for the actual absolute address of RXRAM. Content of this register are valid only when any of the TGINTVECT0 or TGINTVECT1 and SPIFLG registers show an RXOVRN error vector while in Multibuffer mode. If there are multiple Overrun errors, then this register holds address of first overrun address until it is read."]
pub type RxovrnBufAddrR = crate::FieldReader<u16>;
#[doc = "Field `RXOVRN_BUF_ADDR` writer - 10:0\\]
Address of the RAM location of RXRAM for which an Overwrite occured. This address value will show only the offset address of the RAM location in the Multibuffer RAM address space. Refer to the device Spec for the actual absolute address of RXRAM. Content of this register are valid only when any of the TGINTVECT0 or TGINTVECT1 and SPIFLG registers show an RXOVRN error vector while in Multibuffer mode. If there are multiple Overrun errors, then this register holds address of first overrun address until it is read."]
pub type RxovrnBufAddrW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `NU` reader - 31:11\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type NuR = crate::FieldReader<u32>;
#[doc = "Field `NU` writer - 31:11\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl R {
    #[doc = "Bits 0:10 - 10:0\\]
Address of the RAM location of RXRAM for which an Overwrite occured. This address value will show only the offset address of the RAM location in the Multibuffer RAM address space. Refer to the device Spec for the actual absolute address of RXRAM. Content of this register are valid only when any of the TGINTVECT0 or TGINTVECT1 and SPIFLG registers show an RXOVRN error vector while in Multibuffer mode. If there are multiple Overrun errors, then this register holds address of first overrun address until it is read."]
    #[inline(always)]
    pub fn rxovrn_buf_addr(&self) -> RxovrnBufAddrR {
        RxovrnBufAddrR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:31 - 31:11\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:10 - 10:0\\]
Address of the RAM location of RXRAM for which an Overwrite occured. This address value will show only the offset address of the RAM location in the Multibuffer RAM address space. Refer to the device Spec for the actual absolute address of RXRAM. Content of this register are valid only when any of the TGINTVECT0 or TGINTVECT1 and SPIFLG registers show an RXOVRN error vector while in Multibuffer mode. If there are multiple Overrun errors, then this register holds address of first overrun address until it is read."]
    #[inline(always)]
    #[must_use]
    pub fn rxovrn_buf_addr(&mut self) -> RxovrnBufAddrW<RxovrnBufAddrSpec> {
        RxovrnBufAddrW::new(self, 0)
    }
    #[doc = "Bits 11:31 - 31:11\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<RxovrnBufAddrSpec> {
        NuW::new(self, 11)
    }
}
#[doc = "Receive RAM Overrun Buffer Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxovrn_buf_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxovrn_buf_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxovrnBufAddrSpec;
impl crate::RegisterSpec for RxovrnBufAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxovrn_buf_addr::R`](R) reader structure"]
impl crate::Readable for RxovrnBufAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`rxovrn_buf_addr::W`](W) writer structure"]
impl crate::Writable for RxovrnBufAddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXOVRN_BUF_ADDR to value 0"]
impl crate::Resettable for RxovrnBufAddrSpec {
    const RESET_VALUE: u32 = 0;
}
