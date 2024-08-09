#[doc = "Register `SS_CONTROL_REG` reader"]
pub type R = crate::R<SsControlRegSpec>;
#[doc = "Register `SS_CONTROL_REG` writer"]
pub type W = crate::W<SsControlRegSpec>;
#[doc = "Field `ENERGY_EFFICIENT_ETHERNET` reader - 0:0\\]
Energy Efficient Ethernet Enable: 0=EEE is disabled, 1=EEE is enabled"]
pub type EnergyEfficientEthernetR = crate::BitReader;
#[doc = "Field `ENERGY_EFFICIENT_ETHERNET` writer - 0:0\\]
Energy Efficient Ethernet Enable: 0=EEE is disabled, 1=EEE is enabled"]
pub type EnergyEfficientEthernetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENERGY_EFFICIENT_ENABLE` reader - 1:1\\]
Energy Efficient Enable Phy Only Mode: 0=The low power indicate state includes gating off the CPPI_GCLK to the CPSW, 1=The low power indicate state does not gate the clock to the CPSW"]
pub type EnergyEfficientEnableR = crate::BitReader;
#[doc = "Field `ENERGY_EFFICIENT_ENABLE` writer - 1:1\\]
Energy Efficient Enable Phy Only Mode: 0=The low power indicate state includes gating off the CPPI_GCLK to the CPSW, 1=The low power indicate state does not gate the clock to the CPSW"]
pub type EnergyEfficientEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Energy Efficient Ethernet Enable: 0=EEE is disabled, 1=EEE is enabled"]
    #[inline(always)]
    pub fn energy_efficient_ethernet(&self) -> EnergyEfficientEthernetR {
        EnergyEfficientEthernetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Energy Efficient Enable Phy Only Mode: 0=The low power indicate state includes gating off the CPPI_GCLK to the CPSW, 1=The low power indicate state does not gate the clock to the CPSW"]
    #[inline(always)]
    pub fn energy_efficient_enable(&self) -> EnergyEfficientEnableR {
        EnergyEfficientEnableR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Energy Efficient Ethernet Enable: 0=EEE is disabled, 1=EEE is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn energy_efficient_ethernet(&mut self) -> EnergyEfficientEthernetW<SsControlRegSpec> {
        EnergyEfficientEthernetW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Energy Efficient Enable Phy Only Mode: 0=The low power indicate state includes gating off the CPPI_GCLK to the CPSW, 1=The low power indicate state does not gate the clock to the CPSW"]
    #[inline(always)]
    #[must_use]
    pub fn energy_efficient_enable(&mut self) -> EnergyEfficientEnableW<SsControlRegSpec> {
        EnergyEfficientEnableW::new(self, 1)
    }
}
#[doc = "SS Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ss_control_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ss_control_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SsControlRegSpec;
impl crate::RegisterSpec for SsControlRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ss_control_reg::R`](R) reader structure"]
impl crate::Readable for SsControlRegSpec {}
#[doc = "`write(|w| ..)` method takes [`ss_control_reg::W`](W) writer structure"]
impl crate::Writable for SsControlRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SS_CONTROL_REG to value 0"]
impl crate::Resettable for SsControlRegSpec {
    const RESET_VALUE: u32 = 0;
}
