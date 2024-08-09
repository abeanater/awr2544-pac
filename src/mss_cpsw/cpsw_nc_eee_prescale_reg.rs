#[doc = "Register `CPSW_NC_EEE_PRESCALE_REG` reader"]
pub type R = crate::R<CpswNcEeePrescaleRegSpec>;
#[doc = "Register `CPSW_NC_EEE_PRESCALE_REG` writer"]
pub type W = crate::W<CpswNcEeePrescaleRegSpec>;
#[doc = "Field `ENERGY_EFFICIENT_ETHERNET` reader - 11:0\\]
Energy Efficient Ethernet Pre-scale count load value"]
pub type EnergyEfficientEthernetR = crate::FieldReader<u16>;
#[doc = "Field `ENERGY_EFFICIENT_ETHERNET` writer - 11:0\\]
Energy Efficient Ethernet Pre-scale count load value"]
pub type EnergyEfficientEthernetW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Energy Efficient Ethernet Pre-scale count load value"]
    #[inline(always)]
    pub fn energy_efficient_ethernet(&self) -> EnergyEfficientEthernetR {
        EnergyEfficientEthernetR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Energy Efficient Ethernet Pre-scale count load value"]
    #[inline(always)]
    #[must_use]
    pub fn energy_efficient_ethernet(
        &mut self,
    ) -> EnergyEfficientEthernetW<CpswNcEeePrescaleRegSpec> {
        EnergyEfficientEthernetW::new(self, 0)
    }
}
#[doc = "CPSW Energy Efficient Ethernet Prescale Value\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eee_prescale_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eee_prescale_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcEeePrescaleRegSpec;
impl crate::RegisterSpec for CpswNcEeePrescaleRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_eee_prescale_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcEeePrescaleRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_eee_prescale_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcEeePrescaleRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_EEE_PRESCALE_REG to value 0"]
impl crate::Resettable for CpswNcEeePrescaleRegSpec {
    const RESET_VALUE: u32 = 0;
}
