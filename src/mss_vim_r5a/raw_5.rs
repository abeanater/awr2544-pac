#[doc = "Register `RAW_5` reader"]
pub type R = crate::R<Raw5Spec>;
#[doc = "Register `RAW_5` writer"]
pub type W = crate::W<Raw5Spec>;
#[doc = "Field `STS` reader - 31:0\\]
This is the raw status of the events in Group M Each bit corresponds to event Q where Q = Mx32+Bit Read 0 Inactive Read 1 Active/Pending Write 0 No effect Write 1 Set to Interrupt Raw Status"]
pub type StsR = crate::FieldReader<u32>;
#[doc = "Field `STS` writer - 31:0\\]
This is the raw status of the events in Group M Each bit corresponds to event Q where Q = Mx32+Bit Read 0 Inactive Read 1 Active/Pending Write 0 No effect Write 1 Set to Interrupt Raw Status"]
pub type StsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This is the raw status of the events in Group M Each bit corresponds to event Q where Q = Mx32+Bit Read 0 Inactive Read 1 Active/Pending Write 0 No effect Write 1 Set to Interrupt Raw Status"]
    #[inline(always)]
    pub fn sts(&self) -> StsR {
        StsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This is the raw status of the events in Group M Each bit corresponds to event Q where Q = Mx32+Bit Read 0 Inactive Read 1 Active/Pending Write 0 No effect Write 1 Set to Interrupt Raw Status"]
    #[inline(always)]
    #[must_use]
    pub fn sts(&mut self) -> StsW<Raw5Spec> {
        StsW::new(self, 0)
    }
}
#[doc = "Group M Interrupt Raw Status/Set Register (M is 0 to 7) h400 + M x h20 + h00\n\nYou can [`read`](crate::Reg::read) this register and get [`raw_5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`raw_5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Raw5Spec;
impl crate::RegisterSpec for Raw5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`raw_5::R`](R) reader structure"]
impl crate::Readable for Raw5Spec {}
#[doc = "`write(|w| ..)` method takes [`raw_5::W`](W) writer structure"]
impl crate::Writable for Raw5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RAW_5 to value 0"]
impl crate::Resettable for Raw5Spec {
    const RESET_VALUE: u32 = 0;
}
