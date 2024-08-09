#[doc = "Register `BSS_DSS_L3_ACCESS` reader"]
pub type R = crate::R<BssDssL3AccessSpec>;
#[doc = "Register `BSS_DSS_L3_ACCESS` writer"]
pub type W = crate::W<BssDssL3AccessSpec>;
#[doc = "Field `status` reader - 0:0\\]
reading 1'b0: DSS_L3_BANKD1 is not allocated to BSS_TCMA 1'b1: DSS_L3_BANKD1 is allocated to BSS_TCMA"]
pub type StatusR = crate::BitReader;
#[doc = "Field `status` writer - 0:0\\]
reading 1'b0: DSS_L3_BANKD1 is not allocated to BSS_TCMA 1'b1: DSS_L3_BANKD1 is allocated to BSS_TCMA"]
pub type StatusW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
reading 1'b0: DSS_L3_BANKD1 is not allocated to BSS_TCMA 1'b1: DSS_L3_BANKD1 is allocated to BSS_TCMA"]
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
reading 1'b0: DSS_L3_BANKD1 is not allocated to BSS_TCMA 1'b1: DSS_L3_BANKD1 is allocated to BSS_TCMA"]
    #[inline(always)]
    #[must_use]
    pub fn status(&mut self) -> StatusW<BssDssL3AccessSpec> {
        StatusW::new(self, 0)
    }
}
#[doc = "BSS_DSS_L3_ACCESS\n\nYou can [`read`](crate::Reg::read) this register and get [`bss_dss_l3_access::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bss_dss_l3_access::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BssDssL3AccessSpec;
impl crate::RegisterSpec for BssDssL3AccessSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bss_dss_l3_access::R`](R) reader structure"]
impl crate::Readable for BssDssL3AccessSpec {}
#[doc = "`write(|w| ..)` method takes [`bss_dss_l3_access::W`](W) writer structure"]
impl crate::Writable for BssDssL3AccessSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BSS_DSS_L3_ACCESS to value 0"]
impl crate::Resettable for BssDssL3AccessSpec {
    const RESET_VALUE: u32 = 0;
}
