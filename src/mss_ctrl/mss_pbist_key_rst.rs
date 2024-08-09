#[doc = "Register `MSS_PBIST_KEY_RST` reader"]
pub type R = crate::R<MssPbistKeyRstSpec>;
#[doc = "Register `MSS_PBIST_KEY_RST` writer"]
pub type W = crate::W<MssPbistKeyRstSpec>;
#[doc = "Field `pbist_st_key` reader - 3:0\\]
Top PBIST Selftest Key. Valid value is 0x5"]
pub type PbistStKeyR = crate::FieldReader;
#[doc = "Field `pbist_st_key` writer - 3:0\\]
Top PBIST Selftest Key. Valid value is 0x5"]
pub type PbistStKeyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `pbist_st_rst` reader - 7:4\\]
MSS PBIST controller will be brought out of reset when value is 0xA"]
pub type PbistStRstR = crate::FieldReader;
#[doc = "Field `pbist_st_rst` writer - 7:4\\]
MSS PBIST controller will be brought out of reset when value is 0xA"]
pub type PbistStRstW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Top PBIST Selftest Key. Valid value is 0x5"]
    #[inline(always)]
    pub fn pbist_st_key(&self) -> PbistStKeyR {
        PbistStKeyR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
MSS PBIST controller will be brought out of reset when value is 0xA"]
    #[inline(always)]
    pub fn pbist_st_rst(&self) -> PbistStRstR {
        PbistStRstR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Top PBIST Selftest Key. Valid value is 0x5"]
    #[inline(always)]
    #[must_use]
    pub fn pbist_st_key(&mut self) -> PbistStKeyW<MssPbistKeyRstSpec> {
        PbistStKeyW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
MSS PBIST controller will be brought out of reset when value is 0xA"]
    #[inline(always)]
    #[must_use]
    pub fn pbist_st_rst(&mut self) -> PbistStRstW<MssPbistKeyRstSpec> {
        PbistStRstW::new(self, 4)
    }
}
#[doc = "MSS_PBIST_KEY_RST\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_pbist_key_rst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_pbist_key_rst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssPbistKeyRstSpec;
impl crate::RegisterSpec for MssPbistKeyRstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_pbist_key_rst::R`](R) reader structure"]
impl crate::Readable for MssPbistKeyRstSpec {}
#[doc = "`write(|w| ..)` method takes [`mss_pbist_key_rst::W`](W) writer structure"]
impl crate::Writable for MssPbistKeyRstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_PBIST_KEY_RST to value 0"]
impl crate::Resettable for MssPbistKeyRstSpec {
    const RESET_VALUE: u32 = 0;
}
