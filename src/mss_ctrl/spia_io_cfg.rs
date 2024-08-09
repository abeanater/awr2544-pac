#[doc = "Register `SPIA_IO_CFG` reader"]
pub type R = crate::R<SpiaIoCfgSpec>;
#[doc = "Register `SPIA_IO_CFG` writer"]
pub type W = crate::W<SpiaIoCfgSpec>;
#[doc = "Field `cs_deact` reader - 2:0\\]
RESERVED:Dont Use"]
pub type CsDeactR = crate::FieldReader;
#[doc = "Field `cs_deact` writer - 2:0\\]
RESERVED:Dont Use"]
pub type CsDeactW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `cs_pol` reader - 10:8\\]
RESERVED:Dont Use"]
pub type CsPolR = crate::FieldReader;
#[doc = "Field `cs_pol` writer - 10:8\\]
RESERVED:Dont Use"]
pub type CsPolW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `miso_oen_by_cs` reader - 18:16\\]
RESERVED:Dont Use"]
pub type MisoOenByCsR = crate::FieldReader;
#[doc = "Field `miso_oen_by_cs` writer - 18:16\\]
RESERVED:Dont Use"]
pub type MisoOenByCsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
RESERVED:Dont Use"]
    #[inline(always)]
    pub fn cs_deact(&self) -> CsDeactR {
        CsDeactR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
RESERVED:Dont Use"]
    #[inline(always)]
    pub fn cs_pol(&self) -> CsPolR {
        CsPolR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
RESERVED:Dont Use"]
    #[inline(always)]
    pub fn miso_oen_by_cs(&self) -> MisoOenByCsR {
        MisoOenByCsR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
RESERVED:Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn cs_deact(&mut self) -> CsDeactW<SpiaIoCfgSpec> {
        CsDeactW::new(self, 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
RESERVED:Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn cs_pol(&mut self) -> CsPolW<SpiaIoCfgSpec> {
        CsPolW::new(self, 8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
RESERVED:Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn miso_oen_by_cs(&mut self) -> MisoOenByCsW<SpiaIoCfgSpec> {
        MisoOenByCsW::new(self, 16)
    }
}
#[doc = "SPIA_IO_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`spia_io_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spia_io_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiaIoCfgSpec;
impl crate::RegisterSpec for SpiaIoCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spia_io_cfg::R`](R) reader structure"]
impl crate::Readable for SpiaIoCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`spia_io_cfg::W`](W) writer structure"]
impl crate::Writable for SpiaIoCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPIA_IO_CFG to value 0"]
impl crate::Resettable for SpiaIoCfgSpec {
    const RESET_VALUE: u32 = 0;
}
