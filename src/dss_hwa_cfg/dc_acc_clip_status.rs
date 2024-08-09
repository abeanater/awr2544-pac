#[doc = "Register `DC_ACC_CLIP_STATUS` reader"]
pub type R = crate::R<DcAccClipStatusSpec>;
#[doc = "Register `DC_ACC_CLIP_STATUS` writer"]
pub type W = crate::W<DcAccClipStatusSpec>;
#[doc = "Field `dc_acc_clip_status` reader - 11:0\\]
This register contains the clip status of both I/Q of DC accumulators 0 to 11"]
pub type DcAccClipStatusR = crate::FieldReader<u16>;
#[doc = "Field `dc_acc_clip_status` writer - 11:0\\]
This register contains the clip status of both I/Q of DC accumulators 0 to 11"]
pub type DcAccClipStatusW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
This register contains the clip status of both I/Q of DC accumulators 0 to 11"]
    #[inline(always)]
    pub fn dc_acc_clip_status(&self) -> DcAccClipStatusR {
        DcAccClipStatusR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
This register contains the clip status of both I/Q of DC accumulators 0 to 11"]
    #[inline(always)]
    #[must_use]
    pub fn dc_acc_clip_status(&mut self) -> DcAccClipStatusW<DcAccClipStatusSpec> {
        DcAccClipStatusW::new(self, 0)
    }
}
#[doc = "DC_ACC_CLIP_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc_clip_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc_clip_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcAccClipStatusSpec;
impl crate::RegisterSpec for DcAccClipStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc_acc_clip_status::R`](R) reader structure"]
impl crate::Readable for DcAccClipStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`dc_acc_clip_status::W`](W) writer structure"]
impl crate::Writable for DcAccClipStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DC_ACC_CLIP_STATUS to value 0"]
impl crate::Resettable for DcAccClipStatusSpec {
    const RESET_VALUE: u32 = 0;
}
