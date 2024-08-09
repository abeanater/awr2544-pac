#[doc = "Register `MSTIDENA` reader"]
pub type R = crate::R<MstidenaSpec>;
#[doc = "Register `MSTIDENA` writer"]
pub type W = crate::W<MstidenaSpec>;
#[doc = "Field `MSTID_CHK_EN` reader - 3:0\\]
Readable in both user and privileged modes. Writable only in privileged mode 1010 = Enable the master-id feature check. others = Master-id check is disabled."]
pub type MstidChkEnR = crate::FieldReader;
#[doc = "Field `MSTID_CHK_EN` writer - 3:0\\]
Readable in both user and privileged modes. Writable only in privileged mode 1010 = Enable the master-id feature check. others = Master-id check is disabled."]
pub type MstidChkEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Readable in both user and privileged modes. Writable only in privileged mode 1010 = Enable the master-id feature check. others = Master-id check is disabled."]
    #[inline(always)]
    pub fn mstid_chk_en(&self) -> MstidChkEnR {
        MstidChkEnR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Readable in both user and privileged modes. Writable only in privileged mode 1010 = Enable the master-id feature check. others = Master-id check is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn mstid_chk_en(&mut self) -> MstidChkEnW<MstidenaSpec> {
        MstidChkEnW::new(self, 0)
    }
}
#[doc = "MasterID Protection Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mstidena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mstidena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MstidenaSpec;
impl crate::RegisterSpec for MstidenaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mstidena::R`](R) reader structure"]
impl crate::Readable for MstidenaSpec {}
#[doc = "`write(|w| ..)` method takes [`mstidena::W`](W) writer structure"]
impl crate::Writable for MstidenaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSTIDENA to value 0"]
impl crate::Resettable for MstidenaSpec {
    const RESET_VALUE: u32 = 0;
}
