#[doc = "Register `CPSW_NC_CPPI_P0_TX_D_THRESH_SET_L_REG` reader"]
pub type R = crate::R<CpswNcCppiP0TxDThreshSetLRegSpec>;
#[doc = "Register `CPSW_NC_CPPI_P0_TX_D_THRESH_SET_L_REG` writer"]
pub type W = crate::W<CpswNcCppiP0TxDThreshSetLRegSpec>;
#[doc = "Field `PORT_PRIORITY_BASED` reader - 4:0\\]
Port Priority Based Flow Control Threshold Set Value for Priority 0"]
pub type PortPriorityBasedR = crate::FieldReader;
#[doc = "Field `PORT_PRIORITY_BASED` writer - 4:0\\]
Port Priority Based Flow Control Threshold Set Value for Priority 0"]
pub type PortPriorityBasedW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PORT_PRIORITY_BASED` reader - 12:8\\]
Port Priority Based Flow Control Threshold Set Value for Priority 1"]
pub type PortPriorityBasedR = crate::FieldReader;
#[doc = "Field `PORT_PRIORITY_BASED` writer - 12:8\\]
Port Priority Based Flow Control Threshold Set Value for Priority 1"]
pub type PortPriorityBasedW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PORT_PRIORITY_BASED` reader - 20:16\\]
Port Priority Based Flow Control Threshold Set Value for Priority 2"]
pub type PortPriorityBasedR = crate::FieldReader;
#[doc = "Field `PORT_PRIORITY_BASED` writer - 20:16\\]
Port Priority Based Flow Control Threshold Set Value for Priority 2"]
pub type PortPriorityBasedW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PORT_PRIORITY_BASED` reader - 28:24\\]
Port Priority Based Flow Control Threshold Set Value for Priority 3"]
pub type PortPriorityBasedR = crate::FieldReader;
#[doc = "Field `PORT_PRIORITY_BASED` writer - 28:24\\]
Port Priority Based Flow Control Threshold Set Value for Priority 3"]
pub type PortPriorityBasedW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Port Priority Based Flow Control Threshold Set Value for Priority 0"]
    #[inline(always)]
    pub fn port_priority_based(&self) -> PortPriorityBasedR {
        PortPriorityBasedR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Port Priority Based Flow Control Threshold Set Value for Priority 1"]
    #[inline(always)]
    pub fn port_priority_based(&self) -> PortPriorityBasedR {
        PortPriorityBasedR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Port Priority Based Flow Control Threshold Set Value for Priority 2"]
    #[inline(always)]
    pub fn port_priority_based(&self) -> PortPriorityBasedR {
        PortPriorityBasedR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Port Priority Based Flow Control Threshold Set Value for Priority 3"]
    #[inline(always)]
    pub fn port_priority_based(&self) -> PortPriorityBasedR {
        PortPriorityBasedR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Port Priority Based Flow Control Threshold Set Value for Priority 0"]
    #[inline(always)]
    #[must_use]
    pub fn port_priority_based(&mut self) -> PortPriorityBasedW<CpswNcCppiP0TxDThreshSetLRegSpec> {
        PortPriorityBasedW::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Port Priority Based Flow Control Threshold Set Value for Priority 1"]
    #[inline(always)]
    #[must_use]
    pub fn port_priority_based(&mut self) -> PortPriorityBasedW<CpswNcCppiP0TxDThreshSetLRegSpec> {
        PortPriorityBasedW::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Port Priority Based Flow Control Threshold Set Value for Priority 2"]
    #[inline(always)]
    #[must_use]
    pub fn port_priority_based(&mut self) -> PortPriorityBasedW<CpswNcCppiP0TxDThreshSetLRegSpec> {
        PortPriorityBasedW::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Port Priority Based Flow Control Threshold Set Value for Priority 3"]
    #[inline(always)]
    #[must_use]
    pub fn port_priority_based(&mut self) -> PortPriorityBasedW<CpswNcCppiP0TxDThreshSetLRegSpec> {
        PortPriorityBasedW::new(self, 24)
    }
}
#[doc = "CPPI Port 0 Tx PFC Destination Threshold Set Low\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_tx_d_thresh_set_l_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_tx_d_thresh_set_l_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcCppiP0TxDThreshSetLRegSpec;
impl crate::RegisterSpec for CpswNcCppiP0TxDThreshSetLRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_cppi_p0_tx_d_thresh_set_l_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcCppiP0TxDThreshSetLRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_cppi_p0_tx_d_thresh_set_l_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcCppiP0TxDThreshSetLRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_CPPI_P0_TX_D_THRESH_SET_L_REG to value 0"]
impl crate::Resettable for CpswNcCppiP0TxDThreshSetLRegSpec {
    const RESET_VALUE: u32 = 0;
}
