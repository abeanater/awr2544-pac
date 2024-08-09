#[doc = "Register `DC_EST_CLIP_STATUS` reader"]
pub type R = crate::R<DcEstClipStatusSpec>;
#[doc = "Register `DC_EST_CLIP_STATUS` writer"]
pub type W = crate::W<DcEstClipStatusSpec>;
#[doc = "Field `dc_est_clip_status` reader - 11:0\\]
This register contains the clip status of DC estimates (both I &amp; Q combined)"]
pub type DcEstClipStatusR = crate::FieldReader<u16>;
#[doc = "Field `dc_est_clip_status` writer - 11:0\\]
This register contains the clip status of DC estimates (both I &amp; Q combined)"]
pub type DcEstClipStatusW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
This register contains the clip status of DC estimates (both I &amp; Q combined)"]
    #[inline(always)]
    pub fn dc_est_clip_status(&self) -> DcEstClipStatusR {
        DcEstClipStatusR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
This register contains the clip status of DC estimates (both I &amp; Q combined)"]
    #[inline(always)]
    #[must_use]
    pub fn dc_est_clip_status(&mut self) -> DcEstClipStatusW<DcEstClipStatusSpec> {
        DcEstClipStatusW::new(self, 0)
    }
}
#[doc = "DC_EST_CLIP_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_est_clip_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_est_clip_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcEstClipStatusSpec;
impl crate::RegisterSpec for DcEstClipStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc_est_clip_status::R`](R) reader structure"]
impl crate::Readable for DcEstClipStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`dc_est_clip_status::W`](W) writer structure"]
impl crate::Writable for DcEstClipStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DC_EST_CLIP_STATUS to value 0"]
impl crate::Resettable for DcEstClipStatusSpec {
    const RESET_VALUE: u32 = 0;
}
