#[doc = "Register `CPSW_NC_CPPI_P0_PORT_VLAN_REG` reader"]
pub type R = crate::R<CpswNcCppiP0PortVlanRegSpec>;
#[doc = "Register `CPSW_NC_CPPI_P0_PORT_VLAN_REG` writer"]
pub type W = crate::W<CpswNcCppiP0PortVlanRegSpec>;
#[doc = "Field `PORT_VLAN_ID` reader - 11:0\\]
Port VLAN ID"]
pub type PortVlanIdR = crate::FieldReader<u16>;
#[doc = "Field `PORT_VLAN_ID` writer - 11:0\\]
Port VLAN ID"]
pub type PortVlanIdW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `PORT_CFI_BIT` reader - 12:12\\]
Port CFI bit"]
pub type PortCfiBitR = crate::BitReader;
#[doc = "Field `PORT_CFI_BIT` writer - 12:12\\]
Port CFI bit"]
pub type PortCfiBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORT_VLAN_PRIORITY` reader - 15:13\\]
Port VLAN Priority"]
pub type PortVlanPriorityR = crate::FieldReader;
#[doc = "Field `PORT_VLAN_PRIORITY` writer - 15:13\\]
Port VLAN Priority"]
pub type PortVlanPriorityW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Port VLAN ID"]
    #[inline(always)]
    pub fn port_vlan_id(&self) -> PortVlanIdR {
        PortVlanIdR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - 12:12\\]
Port CFI bit"]
    #[inline(always)]
    pub fn port_cfi_bit(&self) -> PortCfiBitR {
        PortCfiBitR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - 15:13\\]
Port VLAN Priority"]
    #[inline(always)]
    pub fn port_vlan_priority(&self) -> PortVlanPriorityR {
        PortVlanPriorityR::new(((self.bits >> 13) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Port VLAN ID"]
    #[inline(always)]
    #[must_use]
    pub fn port_vlan_id(&mut self) -> PortVlanIdW<CpswNcCppiP0PortVlanRegSpec> {
        PortVlanIdW::new(self, 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Port CFI bit"]
    #[inline(always)]
    #[must_use]
    pub fn port_cfi_bit(&mut self) -> PortCfiBitW<CpswNcCppiP0PortVlanRegSpec> {
        PortCfiBitW::new(self, 12)
    }
    #[doc = "Bits 13:15 - 15:13\\]
Port VLAN Priority"]
    #[inline(always)]
    #[must_use]
    pub fn port_vlan_priority(&mut self) -> PortVlanPriorityW<CpswNcCppiP0PortVlanRegSpec> {
        PortVlanPriorityW::new(self, 13)
    }
}
#[doc = "CPPI Port 0 VLAN\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_port_vlan_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_port_vlan_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcCppiP0PortVlanRegSpec;
impl crate::RegisterSpec for CpswNcCppiP0PortVlanRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_cppi_p0_port_vlan_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcCppiP0PortVlanRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_cppi_p0_port_vlan_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcCppiP0PortVlanRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_CPPI_P0_PORT_VLAN_REG to value 0"]
impl crate::Resettable for CpswNcCppiP0PortVlanRegSpec {
    const RESET_VALUE: u32 = 0;
}
