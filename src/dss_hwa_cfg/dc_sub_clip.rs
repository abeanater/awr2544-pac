#[doc = "Register `DC_SUB_CLIP` reader"]
pub type R = crate::R<DcSubClipSpec>;
#[doc = "Register `DC_SUB_CLIP` writer"]
pub type W = crate::W<DcSubClipSpec>;
#[doc = "Field `DC_SUB_CLIP` reader - 0:0\\]
Indicates the DC subtraction clip status"]
pub type DcSubClipR = crate::BitReader;
#[doc = "Field `DC_SUB_CLIP` writer - 0:0\\]
Indicates the DC subtraction clip status"]
pub type DcSubClipW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Indicates the DC subtraction clip status"]
    #[inline(always)]
    pub fn dc_sub_clip(&self) -> DcSubClipR {
        DcSubClipR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Indicates the DC subtraction clip status"]
    #[inline(always)]
    #[must_use]
    pub fn dc_sub_clip(&mut self) -> DcSubClipW<DcSubClipSpec> {
        DcSubClipW::new(self, 0)
    }
}
#[doc = "DC_SUB_CLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_sub_clip::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_sub_clip::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcSubClipSpec;
impl crate::RegisterSpec for DcSubClipSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc_sub_clip::R`](R) reader structure"]
impl crate::Readable for DcSubClipSpec {}
#[doc = "`write(|w| ..)` method takes [`dc_sub_clip::W`](W) writer structure"]
impl crate::Writable for DcSubClipSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DC_SUB_CLIP to value 0"]
impl crate::Resettable for DcSubClipSpec {
    const RESET_VALUE: u32 = 0;
}
