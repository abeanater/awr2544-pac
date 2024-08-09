#[doc = "Register `BSS_DSS_L3_STICKY` reader"]
pub type R = crate::R<BssDssL3StickySpec>;
#[doc = "Register `BSS_DSS_L3_STICKY` writer"]
pub type W = crate::W<BssDssL3StickySpec>;
#[doc = "Field `sticky_enable` reader - 2:0\\]
writing 3'b111 make the BSS_CONTROL::DSS_L3_ACCESS_ENABLE sticky. Further writes to DSS_L3_ACCESS_ENABLE wont impact the register"]
pub type StickyEnableR = crate::FieldReader;
#[doc = "Field `sticky_enable` writer - 2:0\\]
writing 3'b111 make the BSS_CONTROL::DSS_L3_ACCESS_ENABLE sticky. Further writes to DSS_L3_ACCESS_ENABLE wont impact the register"]
pub type StickyEnableW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
writing 3'b111 make the BSS_CONTROL::DSS_L3_ACCESS_ENABLE sticky. Further writes to DSS_L3_ACCESS_ENABLE wont impact the register"]
    #[inline(always)]
    pub fn sticky_enable(&self) -> StickyEnableR {
        StickyEnableR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
writing 3'b111 make the BSS_CONTROL::DSS_L3_ACCESS_ENABLE sticky. Further writes to DSS_L3_ACCESS_ENABLE wont impact the register"]
    #[inline(always)]
    #[must_use]
    pub fn sticky_enable(&mut self) -> StickyEnableW<BssDssL3StickySpec> {
        StickyEnableW::new(self, 0)
    }
}
#[doc = "BSS_DSS_L3_STICKY\n\nYou can [`read`](crate::Reg::read) this register and get [`bss_dss_l3_sticky::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bss_dss_l3_sticky::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BssDssL3StickySpec;
impl crate::RegisterSpec for BssDssL3StickySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bss_dss_l3_sticky::R`](R) reader structure"]
impl crate::Readable for BssDssL3StickySpec {}
#[doc = "`write(|w| ..)` method takes [`bss_dss_l3_sticky::W`](W) writer structure"]
impl crate::Writable for BssDssL3StickySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BSS_DSS_L3_STICKY to value 0"]
impl crate::Resettable for BssDssL3StickySpec {
    const RESET_VALUE: u32 = 0;
}
