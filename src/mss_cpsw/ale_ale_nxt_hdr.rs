#[doc = "Register `ALE_ALE_NXT_HDR` reader"]
pub type R = crate::R<AleAleNxtHdrSpec>;
#[doc = "Register `ALE_ALE_NXT_HDR` writer"]
pub type W = crate::W<AleAleNxtHdrSpec>;
#[doc = "Field `THE_ IIP_NXT_HDR0_IS` reader - 7:0\\]
The ~iip_nxt_hdr0 is the first protocol or next header compared when enabled."]
pub type TheIipNxtHdr0IsR = crate::FieldReader;
#[doc = "Field `THE_ IIP_NXT_HDR0_IS` writer - 7:0\\]
The ~iip_nxt_hdr0 is the first protocol or next header compared when enabled."]
pub type TheIipNxtHdr0IsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `THE_ IIP_NXT_HDR1_IS` reader - 15:8\\]
The ~iip_nxt_hdr1 is the second protocol or next header compared when enabled."]
pub type TheIipNxtHdr1IsR = crate::FieldReader;
#[doc = "Field `THE_ IIP_NXT_HDR1_IS` writer - 15:8\\]
The ~iip_nxt_hdr1 is the second protocol or next header compared when enabled."]
pub type TheIipNxtHdr1IsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `THE_ IIP_NXT_HDR2_IS` reader - 23:16\\]
The ~iip_nxt_hdr2 is the third protocol or next header compared when enabled."]
pub type TheIipNxtHdr2IsR = crate::FieldReader;
#[doc = "Field `THE_ IIP_NXT_HDR2_IS` writer - 23:16\\]
The ~iip_nxt_hdr2 is the third protocol or next header compared when enabled."]
pub type TheIipNxtHdr2IsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `THE_ IIP_NXT_HDR3_IS` reader - 31:24\\]
The ~iip_nxt_hdr3 is the forth protocol or next header compared when enabled."]
pub type TheIipNxtHdr3IsR = crate::FieldReader;
#[doc = "Field `THE_ IIP_NXT_HDR3_IS` writer - 31:24\\]
The ~iip_nxt_hdr3 is the forth protocol or next header compared when enabled."]
pub type TheIipNxtHdr3IsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
The ~iip_nxt_hdr0 is the first protocol or next header compared when enabled."]
    #[inline(always)]
    pub fn the_iip_nxt_hdr0_is(&self) -> TheIipNxtHdr0IsR {
        TheIipNxtHdr0IsR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
The ~iip_nxt_hdr1 is the second protocol or next header compared when enabled."]
    #[inline(always)]
    pub fn the_iip_nxt_hdr1_is(&self) -> TheIipNxtHdr1IsR {
        TheIipNxtHdr1IsR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
The ~iip_nxt_hdr2 is the third protocol or next header compared when enabled."]
    #[inline(always)]
    pub fn the_iip_nxt_hdr2_is(&self) -> TheIipNxtHdr2IsR {
        TheIipNxtHdr2IsR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
The ~iip_nxt_hdr3 is the forth protocol or next header compared when enabled."]
    #[inline(always)]
    pub fn the_iip_nxt_hdr3_is(&self) -> TheIipNxtHdr3IsR {
        TheIipNxtHdr3IsR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
The ~iip_nxt_hdr0 is the first protocol or next header compared when enabled."]
    #[inline(always)]
    #[must_use]
    pub fn the_iip_nxt_hdr0_is(&mut self) -> TheIipNxtHdr0IsW<AleAleNxtHdrSpec> {
        TheIipNxtHdr0IsW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
The ~iip_nxt_hdr1 is the second protocol or next header compared when enabled."]
    #[inline(always)]
    #[must_use]
    pub fn the_iip_nxt_hdr1_is(&mut self) -> TheIipNxtHdr1IsW<AleAleNxtHdrSpec> {
        TheIipNxtHdr1IsW::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
The ~iip_nxt_hdr2 is the third protocol or next header compared when enabled."]
    #[inline(always)]
    #[must_use]
    pub fn the_iip_nxt_hdr2_is(&mut self) -> TheIipNxtHdr2IsW<AleAleNxtHdrSpec> {
        TheIipNxtHdr2IsW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
The ~iip_nxt_hdr3 is the forth protocol or next header compared when enabled."]
    #[inline(always)]
    #[must_use]
    pub fn the_iip_nxt_hdr3_is(&mut self) -> TheIipNxtHdr3IsW<AleAleNxtHdrSpec> {
        TheIipNxtHdr3IsW::new(self, 24)
    }
}
#[doc = "The ALE Next Header is used to limit the IPv6 Next header or IPv4 Protocol values found in the IP header. It is enabled via the ~iLmtNxtHdr bit in the VLAN entry. All four ~iip_nxt_hdr0-3 are compared when enabled, so if only one is required, set them all to the one value to be tested.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_ale_nxt_hdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_ale_nxt_hdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AleAleNxtHdrSpec;
impl crate::RegisterSpec for AleAleNxtHdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ale_ale_nxt_hdr::R`](R) reader structure"]
impl crate::Readable for AleAleNxtHdrSpec {}
#[doc = "`write(|w| ..)` method takes [`ale_ale_nxt_hdr::W`](W) writer structure"]
impl crate::Writable for AleAleNxtHdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALE_ALE_NXT_HDR to value 0"]
impl crate::Resettable for AleAleNxtHdrSpec {
    const RESET_VALUE: u32 = 0;
}
