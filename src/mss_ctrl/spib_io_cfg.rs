#[doc = "Register `SPIB_IO_CFG` reader"]
pub type R = crate::R<SpibIoCfgSpec>;
#[doc = "Register `SPIB_IO_CFG` writer"]
pub type W = crate::W<SpibIoCfgSpec>;
#[doc = "Field `cs_deact` reader - 2:0\\]
1 : MIBSPIB External chip select is overridden with the value of MIBSPIB CS polarity-slave mode"]
pub type CsDeactR = crate::FieldReader;
#[doc = "Field `cs_deact` writer - 2:0\\]
1 : MIBSPIB External chip select is overridden with the value of MIBSPIB CS polarity-slave mode"]
pub type CsDeactW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `cs_pol` reader - 10:8\\]
MIBSPIB CS polarity-slave mode 1: Active high 0:Active low"]
pub type CsPolR = crate::FieldReader;
#[doc = "Field `cs_pol` writer - 10:8\\]
MIBSPIB CS polarity-slave mode 1: Active high 0:Active low"]
pub type CsPolW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `miso_oen_by_cs` reader - 18:16\\]
MIBSPIB MISO OE_N Control based on Chip selectCS-applicable in slave mode 1:MISO OEN controlled based on CS.When CS is inactive OE_N=1 0:MISO OEN controlled by IP"]
pub type MisoOenByCsR = crate::FieldReader;
#[doc = "Field `miso_oen_by_cs` writer - 18:16\\]
MIBSPIB MISO OE_N Control based on Chip selectCS-applicable in slave mode 1:MISO OEN controlled based on CS.When CS is inactive OE_N=1 0:MISO OEN controlled by IP"]
pub type MisoOenByCsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
1 : MIBSPIB External chip select is overridden with the value of MIBSPIB CS polarity-slave mode"]
    #[inline(always)]
    pub fn cs_deact(&self) -> CsDeactR {
        CsDeactR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
MIBSPIB CS polarity-slave mode 1: Active high 0:Active low"]
    #[inline(always)]
    pub fn cs_pol(&self) -> CsPolR {
        CsPolR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
MIBSPIB MISO OE_N Control based on Chip selectCS-applicable in slave mode 1:MISO OEN controlled based on CS.When CS is inactive OE_N=1 0:MISO OEN controlled by IP"]
    #[inline(always)]
    pub fn miso_oen_by_cs(&self) -> MisoOenByCsR {
        MisoOenByCsR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
1 : MIBSPIB External chip select is overridden with the value of MIBSPIB CS polarity-slave mode"]
    #[inline(always)]
    #[must_use]
    pub fn cs_deact(&mut self) -> CsDeactW<SpibIoCfgSpec> {
        CsDeactW::new(self, 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
MIBSPIB CS polarity-slave mode 1: Active high 0:Active low"]
    #[inline(always)]
    #[must_use]
    pub fn cs_pol(&mut self) -> CsPolW<SpibIoCfgSpec> {
        CsPolW::new(self, 8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
MIBSPIB MISO OE_N Control based on Chip selectCS-applicable in slave mode 1:MISO OEN controlled based on CS.When CS is inactive OE_N=1 0:MISO OEN controlled by IP"]
    #[inline(always)]
    #[must_use]
    pub fn miso_oen_by_cs(&mut self) -> MisoOenByCsW<SpibIoCfgSpec> {
        MisoOenByCsW::new(self, 16)
    }
}
#[doc = "SPIB_IO_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`spib_io_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spib_io_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpibIoCfgSpec;
impl crate::RegisterSpec for SpibIoCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spib_io_cfg::R`](R) reader structure"]
impl crate::Readable for SpibIoCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`spib_io_cfg::W`](W) writer structure"]
impl crate::Writable for SpibIoCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPIB_IO_CFG to value 0"]
impl crate::Resettable for SpibIoCfgSpec {
    const RESET_VALUE: u32 = 0;
}
