#[doc = "Register `RST_ASSERDLY` reader"]
pub type R = crate::R<RstAsserdlySpec>;
#[doc = "Register `RST_ASSERDLY` writer"]
pub type W = crate::W<RstAsserdlySpec>;
#[doc = "Field `common` reader - 7:0\\]
Value decides number of cycles reset should be asserted for CR5SS related resets"]
pub type CommonR = crate::FieldReader;
#[doc = "Field `common` writer - 7:0\\]
Value decides number of cycles reset should be asserted for CR5SS related resets"]
pub type CommonW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Value decides number of cycles reset should be asserted for CR5SS related resets"]
    #[inline(always)]
    pub fn common(&self) -> CommonR {
        CommonR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Value decides number of cycles reset should be asserted for CR5SS related resets"]
    #[inline(always)]
    #[must_use]
    pub fn common(&mut self) -> CommonW<RstAsserdlySpec> {
        CommonW::new(self, 0)
    }
}
#[doc = "RST_ASSERDLY\n\nYou can [`read`](crate::Reg::read) this register and get [`rst_asserdly::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst_asserdly::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstAsserdlySpec;
impl crate::RegisterSpec for RstAsserdlySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rst_asserdly::R`](R) reader structure"]
impl crate::Readable for RstAsserdlySpec {}
#[doc = "`write(|w| ..)` method takes [`rst_asserdly::W`](W) writer structure"]
impl crate::Writable for RstAsserdlySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RST_ASSERDLY to value 0"]
impl crate::Resettable for RstAsserdlySpec {
    const RESET_VALUE: u32 = 0;
}
