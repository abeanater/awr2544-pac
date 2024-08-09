#[doc = "Register `DSS_PBIST_KEY_RESET` reader"]
pub type R = crate::R<DssPbistKeyResetSpec>;
#[doc = "Register `DSS_PBIST_KEY_RESET` writer"]
pub type W = crate::W<DssPbistKeyResetSpec>;
#[doc = "Field `dss_pbist_st_key` reader - 3:0\\]
DSS PBIST Selftest Key. Valid value is 0x5"]
pub type DssPbistStKeyR = crate::FieldReader;
#[doc = "Field `dss_pbist_st_key` writer - 3:0\\]
DSS PBIST Selftest Key. Valid value is 0x5"]
pub type DssPbistStKeyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dss_pbist_st_reset` reader - 7:4\\]
DSS PBIST controller will be brought out of reset when value is 0xA"]
pub type DssPbistStResetR = crate::FieldReader;
#[doc = "Field `dss_pbist_st_reset` writer - 7:4\\]
DSS PBIST controller will be brought out of reset when value is 0xA"]
pub type DssPbistStResetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dss_l2_pbist_st_key` reader - 8:8\\]
DSS L2 PBIST Selftest Key."]
pub type DssL2PbistStKeyR = crate::BitReader;
#[doc = "Field `dss_l2_pbist_st_key` writer - 8:8\\]
DSS L2 PBIST Selftest Key."]
pub type DssL2PbistStKeyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
DSS PBIST Selftest Key. Valid value is 0x5"]
    #[inline(always)]
    pub fn dss_pbist_st_key(&self) -> DssPbistStKeyR {
        DssPbistStKeyR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
DSS PBIST controller will be brought out of reset when value is 0xA"]
    #[inline(always)]
    pub fn dss_pbist_st_reset(&self) -> DssPbistStResetR {
        DssPbistStResetR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
DSS L2 PBIST Selftest Key."]
    #[inline(always)]
    pub fn dss_l2_pbist_st_key(&self) -> DssL2PbistStKeyR {
        DssL2PbistStKeyR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
DSS PBIST Selftest Key. Valid value is 0x5"]
    #[inline(always)]
    #[must_use]
    pub fn dss_pbist_st_key(&mut self) -> DssPbistStKeyW<DssPbistKeyResetSpec> {
        DssPbistStKeyW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
DSS PBIST controller will be brought out of reset when value is 0xA"]
    #[inline(always)]
    #[must_use]
    pub fn dss_pbist_st_reset(&mut self) -> DssPbistStResetW<DssPbistKeyResetSpec> {
        DssPbistStResetW::new(self, 4)
    }
    #[doc = "Bit 8 - 8:8\\]
DSS L2 PBIST Selftest Key."]
    #[inline(always)]
    #[must_use]
    pub fn dss_l2_pbist_st_key(&mut self) -> DssL2PbistStKeyW<DssPbistKeyResetSpec> {
        DssL2PbistStKeyW::new(self, 8)
    }
}
#[doc = "DSS_PBIST_KEY_RESET\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_pbist_key_reset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_pbist_key_reset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DssPbistKeyResetSpec;
impl crate::RegisterSpec for DssPbistKeyResetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dss_pbist_key_reset::R`](R) reader structure"]
impl crate::Readable for DssPbistKeyResetSpec {}
#[doc = "`write(|w| ..)` method takes [`dss_pbist_key_reset::W`](W) writer structure"]
impl crate::Writable for DssPbistKeyResetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSS_PBIST_KEY_RESET to value 0"]
impl crate::Resettable for DssPbistKeyResetSpec {
    const RESET_VALUE: u32 = 0;
}
