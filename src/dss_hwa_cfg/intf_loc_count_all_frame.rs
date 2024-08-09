#[doc = "Register `INTF_LOC_COUNT_ALL_FRAME` reader"]
pub type R = crate::R<IntfLocCountAllFrameSpec>;
#[doc = "Register `INTF_LOC_COUNT_ALL_FRAME` writer"]
pub type W = crate::W<IntfLocCountAllFrameSpec>;
#[doc = "Field `intf_loc_count_all_frame` reader - 19:0\\]
Number of samples that exceeded the threshold in a frame"]
pub type IntfLocCountAllFrameR = crate::FieldReader<u32>;
#[doc = "Field `intf_loc_count_all_frame` writer - 19:0\\]
Number of samples that exceeded the threshold in a frame"]
pub type IntfLocCountAllFrameW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - 19:0\\]
Number of samples that exceeded the threshold in a frame"]
    #[inline(always)]
    pub fn intf_loc_count_all_frame(&self) -> IntfLocCountAllFrameR {
        IntfLocCountAllFrameR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - 19:0\\]
Number of samples that exceeded the threshold in a frame"]
    #[inline(always)]
    #[must_use]
    pub fn intf_loc_count_all_frame(&mut self) -> IntfLocCountAllFrameW<IntfLocCountAllFrameSpec> {
        IntfLocCountAllFrameW::new(self, 0)
    }
}
#[doc = "INTF_LOC_COUNT_ALL_FRAME\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_loc_count_all_frame::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_loc_count_all_frame::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfLocCountAllFrameSpec;
impl crate::RegisterSpec for IntfLocCountAllFrameSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf_loc_count_all_frame::R`](R) reader structure"]
impl crate::Readable for IntfLocCountAllFrameSpec {}
#[doc = "`write(|w| ..)` method takes [`intf_loc_count_all_frame::W`](W) writer structure"]
impl crate::Writable for IntfLocCountAllFrameSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTF_LOC_COUNT_ALL_FRAME to value 0"]
impl crate::Resettable for IntfLocCountAllFrameSpec {
    const RESET_VALUE: u32 = 0;
}
