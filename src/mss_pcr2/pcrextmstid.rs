#[doc = "Register `PCREXTMSTID` reader"]
pub type R = crate::R<PcrextmstidSpec>;
#[doc = "Register `PCREXTMSTID` writer"]
pub type W = crate::W<PcrextmstidSpec>;
#[doc = "Field `PCREXT_MSTID` reader - 31:0\\]
These bits sets the permission for maximum of 16 masters to address the external PCR frame. The scheme is similar to the one described for PCSm MSTID in section 1.7.33. Readable in both user and privileged modes. 1 = The memory mapped in respective frames can be addressed by master with matching Master-ID. 0 = The memory is locked for master with matching Master-ID. PCR responds with AERROR. Writable only in privileged mode 1 = Sets the corresponding bit. 0 = Clears the corresponding bit. Writes to unimplemented bits have no effect and reads yield 0."]
pub type PcrextMstidR = crate::FieldReader<u32>;
#[doc = "Field `PCREXT_MSTID` writer - 31:0\\]
These bits sets the permission for maximum of 16 masters to address the external PCR frame. The scheme is similar to the one described for PCSm MSTID in section 1.7.33. Readable in both user and privileged modes. 1 = The memory mapped in respective frames can be addressed by master with matching Master-ID. 0 = The memory is locked for master with matching Master-ID. PCR responds with AERROR. Writable only in privileged mode 1 = Sets the corresponding bit. 0 = Clears the corresponding bit. Writes to unimplemented bits have no effect and reads yield 0."]
pub type PcrextMstidW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
These bits sets the permission for maximum of 16 masters to address the external PCR frame. The scheme is similar to the one described for PCSm MSTID in section 1.7.33. Readable in both user and privileged modes. 1 = The memory mapped in respective frames can be addressed by master with matching Master-ID. 0 = The memory is locked for master with matching Master-ID. PCR responds with AERROR. Writable only in privileged mode 1 = Sets the corresponding bit. 0 = Clears the corresponding bit. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    pub fn pcrext_mstid(&self) -> PcrextMstidR {
        PcrextMstidR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
These bits sets the permission for maximum of 16 masters to address the external PCR frame. The scheme is similar to the one described for PCSm MSTID in section 1.7.33. Readable in both user and privileged modes. 1 = The memory mapped in respective frames can be addressed by master with matching Master-ID. 0 = The memory is locked for master with matching Master-ID. PCR responds with AERROR. Writable only in privileged mode 1 = Sets the corresponding bit. 0 = Clears the corresponding bit. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    #[must_use]
    pub fn pcrext_mstid(&mut self) -> PcrextMstidW<PcrextmstidSpec> {
        PcrextMstidW::new(self, 0)
    }
}
#[doc = "Master-ID Protection Register for external PCR\n\nYou can [`read`](crate::Reg::read) this register and get [`pcrextmstid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrextmstid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcrextmstidSpec;
impl crate::RegisterSpec for PcrextmstidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcrextmstid::R`](R) reader structure"]
impl crate::Readable for PcrextmstidSpec {}
#[doc = "`write(|w| ..)` method takes [`pcrextmstid::W`](W) writer structure"]
impl crate::Writable for PcrextmstidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCREXTMSTID to value 0"]
impl crate::Resettable for PcrextmstidSpec {
    const RESET_VALUE: u32 = 0;
}
