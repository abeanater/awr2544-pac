#[doc = "Register `PBIST_ID` reader"]
pub type R = crate::R<PbistIdSpec>;
#[doc = "Register `PBIST_ID` writer"]
pub type W = crate::W<PbistIdSpec>;
#[doc = "Field `PBIST_ID` reader - 4:0\\]
PBIST ID. This is a unique ID assigned to each PBIST controller in a device with multiple PBIST controllers. The value of this register does not affect the functionality of the CPU interface."]
pub type PbistIdR = crate::FieldReader;
#[doc = "Field `PBIST_ID` writer - 4:0\\]
PBIST ID. This is a unique ID assigned to each PBIST controller in a device with multiple PBIST controllers. The value of this register does not affect the functionality of the CPU interface."]
pub type PbistIdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
PBIST ID. This is a unique ID assigned to each PBIST controller in a device with multiple PBIST controllers. The value of this register does not affect the functionality of the CPU interface."]
    #[inline(always)]
    pub fn pbist_id(&self) -> PbistIdR {
        PbistIdR::new(self.bits & 0x1f)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
PBIST ID. This is a unique ID assigned to each PBIST controller in a device with multiple PBIST controllers. The value of this register does not affect the functionality of the CPU interface."]
    #[inline(always)]
    #[must_use]
    pub fn pbist_id(&mut self) -> PbistIdW<PbistIdSpec> {
        PbistIdW::new(self, 0)
    }
}
#[doc = "PBIST ID\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_id::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_id::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbistIdSpec;
impl crate::RegisterSpec for PbistIdSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pbist_id::R`](R) reader structure"]
impl crate::Readable for PbistIdSpec {}
#[doc = "`write(|w| ..)` method takes [`pbist_id::W`](W) writer structure"]
impl crate::Writable for PbistIdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PBIST_ID to value 0"]
impl crate::Resettable for PbistIdSpec {
    const RESET_VALUE: u8 = 0;
}
