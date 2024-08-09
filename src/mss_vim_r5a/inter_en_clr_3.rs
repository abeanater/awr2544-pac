#[doc = "Register `INTER_EN_CLR_3` reader"]
pub type R = crate::R<InterEnClr3Spec>;
#[doc = "Register `INTER_EN_CLR_3` writer"]
pub type W = crate::W<InterEnClr3Spec>;
#[doc = "Field `MASK` reader - 31:0\\]
This field is used to disable the mask of events in Group M Each bit corresponds to event Q where Q = Mx32+Bit Read 0 Disabled Read 1 Enabled Write 0 No effect Write 1 Clear Enable"]
pub type MaskR = crate::FieldReader<u32>;
#[doc = "Field `MASK` writer - 31:0\\]
This field is used to disable the mask of events in Group M Each bit corresponds to event Q where Q = Mx32+Bit Read 0 Disabled Read 1 Enabled Write 0 No effect Write 1 Clear Enable"]
pub type MaskW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This field is used to disable the mask of events in Group M Each bit corresponds to event Q where Q = Mx32+Bit Read 0 Disabled Read 1 Enabled Write 0 No effect Write 1 Clear Enable"]
    #[inline(always)]
    pub fn mask(&self) -> MaskR {
        MaskR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This field is used to disable the mask of events in Group M Each bit corresponds to event Q where Q = Mx32+Bit Read 0 Disabled Read 1 Enabled Write 0 No effect Write 1 Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mask(&mut self) -> MaskW<InterEnClr3Spec> {
        MaskW::new(self, 0)
    }
}
#[doc = "Group M Interrupt Enabled Clear Register (M is 0 to 7) h400 + M x h20 + h0C\n\nYou can [`read`](crate::Reg::read) this register and get [`inter_en_clr_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inter_en_clr_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InterEnClr3Spec;
impl crate::RegisterSpec for InterEnClr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inter_en_clr_3::R`](R) reader structure"]
impl crate::Readable for InterEnClr3Spec {}
#[doc = "`write(|w| ..)` method takes [`inter_en_clr_3::W`](W) writer structure"]
impl crate::Writable for InterEnClr3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTER_EN_CLR_3 to value 0"]
impl crate::Resettable for InterEnClr3Spec {
    const RESET_VALUE: u32 = 0;
}
