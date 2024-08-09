#[doc = "Register `SS_STATUS_REG` reader"]
pub type R = crate::R<SsStatusRegSpec>;
#[doc = "Register `SS_STATUS_REG` writer"]
pub type W = crate::W<SsStatusRegSpec>;
#[doc = "Field `ENERGY_EFFICIENT_ETHERNET` reader - 0:0\\]
Energy Efficient Ethernet clockstop acknowledge from CPSW"]
pub type EnergyEfficientEthernetR = crate::BitReader;
#[doc = "Field `ENERGY_EFFICIENT_ETHERNET` writer - 0:0\\]
Energy Efficient Ethernet clockstop acknowledge from CPSW"]
pub type EnergyEfficientEthernetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Energy Efficient Ethernet clockstop acknowledge from CPSW"]
    #[inline(always)]
    pub fn energy_efficient_ethernet(&self) -> EnergyEfficientEthernetR {
        EnergyEfficientEthernetR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Energy Efficient Ethernet clockstop acknowledge from CPSW"]
    #[inline(always)]
    #[must_use]
    pub fn energy_efficient_ethernet(&mut self) -> EnergyEfficientEthernetW<SsStatusRegSpec> {
        EnergyEfficientEthernetW::new(self, 0)
    }
}
#[doc = "SS Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ss_status_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ss_status_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SsStatusRegSpec;
impl crate::RegisterSpec for SsStatusRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ss_status_reg::R`](R) reader structure"]
impl crate::Readable for SsStatusRegSpec {}
#[doc = "`write(|w| ..)` method takes [`ss_status_reg::W`](W) writer structure"]
impl crate::Writable for SsStatusRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SS_STATUS_REG to value 0"]
impl crate::Resettable for SsStatusRegSpec {
    const RESET_VALUE: u32 = 0;
}
