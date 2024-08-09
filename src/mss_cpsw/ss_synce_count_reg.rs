#[doc = "Register `SS_SYNCE_COUNT_REG` reader"]
pub type R = crate::R<SsSynceCountRegSpec>;
#[doc = "Register `SS_SYNCE_COUNT_REG` writer"]
pub type W = crate::W<SsSynceCountRegSpec>;
#[doc = "Field `SYNC_E_COUNT` reader - 31:0\\]
Sync E Count Value"]
pub type SyncECountR = crate::FieldReader<u32>;
#[doc = "Field `SYNC_E_COUNT` writer - 31:0\\]
Sync E Count Value"]
pub type SyncECountW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Sync E Count Value"]
    #[inline(always)]
    pub fn sync_e_count(&self) -> SyncECountR {
        SyncECountR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Sync E Count Value"]
    #[inline(always)]
    #[must_use]
    pub fn sync_e_count(&mut self) -> SyncECountW<SsSynceCountRegSpec> {
        SyncECountW::new(self, 0)
    }
}
#[doc = "SS SYNCE Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ss_synce_count_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ss_synce_count_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SsSynceCountRegSpec;
impl crate::RegisterSpec for SsSynceCountRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ss_synce_count_reg::R`](R) reader structure"]
impl crate::Readable for SsSynceCountRegSpec {}
#[doc = "`write(|w| ..)` method takes [`ss_synce_count_reg::W`](W) writer structure"]
impl crate::Writable for SsSynceCountRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SS_SYNCE_COUNT_REG to value 0"]
impl crate::Resettable for SsSynceCountRegSpec {
    const RESET_VALUE: u32 = 0;
}
