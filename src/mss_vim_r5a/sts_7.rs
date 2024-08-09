#[doc = "Register `STS_7` reader"]
pub type R = crate::R<Sts7Spec>;
#[doc = "Register `STS_7` writer"]
pub type W = crate::W<Sts7Spec>;
#[doc = "Field `MASK` reader - 31:0\\]
This is the masked status of the events in Group M Each bit corresponds to event Q where Q = Mx32+Bit Read 0 Inactive or Disabled Read 1 Active/Pending and Enabled Write 0 No effect Write 1 Clear Interrupt Raw Status"]
pub type MaskR = crate::FieldReader<u32>;
#[doc = "Field `MASK` writer - 31:0\\]
This is the masked status of the events in Group M Each bit corresponds to event Q where Q = Mx32+Bit Read 0 Inactive or Disabled Read 1 Active/Pending and Enabled Write 0 No effect Write 1 Clear Interrupt Raw Status"]
pub type MaskW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This is the masked status of the events in Group M Each bit corresponds to event Q where Q = Mx32+Bit Read 0 Inactive or Disabled Read 1 Active/Pending and Enabled Write 0 No effect Write 1 Clear Interrupt Raw Status"]
    #[inline(always)]
    pub fn mask(&self) -> MaskR {
        MaskR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This is the masked status of the events in Group M Each bit corresponds to event Q where Q = Mx32+Bit Read 0 Inactive or Disabled Read 1 Active/Pending and Enabled Write 0 No effect Write 1 Clear Interrupt Raw Status"]
    #[inline(always)]
    #[must_use]
    pub fn mask(&mut self) -> MaskW<Sts7Spec> {
        MaskW::new(self, 0)
    }
}
#[doc = "Group M Interrupt Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h04\n\nYou can [`read`](crate::Reg::read) this register and get [`sts_7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sts_7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sts7Spec;
impl crate::RegisterSpec for Sts7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts_7::R`](R) reader structure"]
impl crate::Readable for Sts7Spec {}
#[doc = "`write(|w| ..)` method takes [`sts_7::W`](W) writer structure"]
impl crate::Writable for Sts7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STS_7 to value 0"]
impl crate::Resettable for Sts7Spec {
    const RESET_VALUE: u32 = 0;
}
