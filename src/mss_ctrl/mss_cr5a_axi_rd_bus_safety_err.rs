#[doc = "Register `MSS_CR5A_AXI_RD_BUS_SAFETY_ERR` reader"]
pub type R = crate::R<MssCr5aAxiRdBusSafetyErrSpec>;
#[doc = "Register `MSS_CR5A_AXI_RD_BUS_SAFETY_ERR` writer"]
pub type W = crate::W<MssCr5aAxiRdBusSafetyErrSpec>;
#[doc = "Field `comp_err` reader - 7:0\\]
Refer to TPR12 Substem Microarch document for more details"]
pub type CompErrR = crate::FieldReader;
#[doc = "Field `comp_err` writer - 7:0\\]
Refer to TPR12 Substem Microarch document for more details"]
pub type CompErrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `comp_check` reader - 15:8\\]
Refer to TPR12 Substem Microarch document for more details"]
pub type CompCheckR = crate::FieldReader;
#[doc = "Field `comp_check` writer - 15:8\\]
Refer to TPR12 Substem Microarch document for more details"]
pub type CompCheckW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `sec` reader - 23:16\\]
Refer to TPR12 Substem Microarch document for more details"]
pub type SecR = crate::FieldReader;
#[doc = "Field `sec` writer - 23:16\\]
Refer to TPR12 Substem Microarch document for more details"]
pub type SecW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ded` reader - 31:24\\]
Refer to TPR12 Substem Microarch document for more details"]
pub type DedR = crate::FieldReader;
#[doc = "Field `ded` writer - 31:24\\]
Refer to TPR12 Substem Microarch document for more details"]
pub type DedW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Refer to TPR12 Substem Microarch document for more details"]
    #[inline(always)]
    pub fn comp_err(&self) -> CompErrR {
        CompErrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Refer to TPR12 Substem Microarch document for more details"]
    #[inline(always)]
    pub fn comp_check(&self) -> CompCheckR {
        CompCheckR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Refer to TPR12 Substem Microarch document for more details"]
    #[inline(always)]
    pub fn sec(&self) -> SecR {
        SecR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Refer to TPR12 Substem Microarch document for more details"]
    #[inline(always)]
    pub fn ded(&self) -> DedR {
        DedR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Refer to TPR12 Substem Microarch document for more details"]
    #[inline(always)]
    #[must_use]
    pub fn comp_err(&mut self) -> CompErrW<MssCr5aAxiRdBusSafetyErrSpec> {
        CompErrW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Refer to TPR12 Substem Microarch document for more details"]
    #[inline(always)]
    #[must_use]
    pub fn comp_check(&mut self) -> CompCheckW<MssCr5aAxiRdBusSafetyErrSpec> {
        CompCheckW::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Refer to TPR12 Substem Microarch document for more details"]
    #[inline(always)]
    #[must_use]
    pub fn sec(&mut self) -> SecW<MssCr5aAxiRdBusSafetyErrSpec> {
        SecW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Refer to TPR12 Substem Microarch document for more details"]
    #[inline(always)]
    #[must_use]
    pub fn ded(&mut self) -> DedW<MssCr5aAxiRdBusSafetyErrSpec> {
        DedW::new(self, 24)
    }
}
#[doc = "MSS_CR5A_AXI_RD_BUS_SAFETY_ERR\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_cr5a_axi_rd_bus_safety_err::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_cr5a_axi_rd_bus_safety_err::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssCr5aAxiRdBusSafetyErrSpec;
impl crate::RegisterSpec for MssCr5aAxiRdBusSafetyErrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_cr5a_axi_rd_bus_safety_err::R`](R) reader structure"]
impl crate::Readable for MssCr5aAxiRdBusSafetyErrSpec {}
#[doc = "`write(|w| ..)` method takes [`mss_cr5a_axi_rd_bus_safety_err::W`](W) writer structure"]
impl crate::Writable for MssCr5aAxiRdBusSafetyErrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_CR5A_AXI_RD_BUS_SAFETY_ERR to value 0"]
impl crate::Resettable for MssCr5aAxiRdBusSafetyErrSpec {
    const RESET_VALUE: u32 = 0;
}
