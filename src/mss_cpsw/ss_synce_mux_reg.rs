#[doc = "Register `SS_SYNCE_MUX_REG` reader"]
pub type R = crate::R<SsSynceMuxRegSpec>;
#[doc = "Register `SS_SYNCE_MUX_REG` writer"]
pub type W = crate::W<SsSynceMuxRegSpec>;
#[doc = "Field `SYNC_E_SELECT` reader - 5:0\\]
Sync E Select Value"]
pub type SyncESelectR = crate::FieldReader;
#[doc = "Field `SYNC_E_SELECT` writer - 5:0\\]
Sync E Select Value"]
pub type SyncESelectW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Sync E Select Value"]
    #[inline(always)]
    pub fn sync_e_select(&self) -> SyncESelectR {
        SyncESelectR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Sync E Select Value"]
    #[inline(always)]
    #[must_use]
    pub fn sync_e_select(&mut self) -> SyncESelectW<SsSynceMuxRegSpec> {
        SyncESelectW::new(self, 0)
    }
}
#[doc = "SS Synce Mux Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ss_synce_mux_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ss_synce_mux_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SsSynceMuxRegSpec;
impl crate::RegisterSpec for SsSynceMuxRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ss_synce_mux_reg::R`](R) reader structure"]
impl crate::Readable for SsSynceMuxRegSpec {}
#[doc = "`write(|w| ..)` method takes [`ss_synce_mux_reg::W`](W) writer structure"]
impl crate::Writable for SsSynceMuxRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SS_SYNCE_MUX_REG to value 0"]
impl crate::Resettable for SsSynceMuxRegSpec {
    const RESET_VALUE: u32 = 0;
}
