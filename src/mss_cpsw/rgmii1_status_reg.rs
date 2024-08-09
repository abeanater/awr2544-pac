#[doc = "Register `RGMII1_STATUS_REG` reader"]
pub type R = crate::R<Rgmii1StatusRegSpec>;
#[doc = "Register `RGMII1_STATUS_REG` writer"]
pub type W = crate::W<Rgmii1StatusRegSpec>;
#[doc = "Field `RGMII1_LINK_INDICATOR` reader - 0:0\\]
Rgmii1 link indicator: 0=Link is down, 1=Link is up"]
pub type Rgmii1LinkIndicatorR = crate::BitReader;
#[doc = "Field `RGMII1_LINK_INDICATOR` writer - 0:0\\]
Rgmii1 link indicator: 0=Link is down, 1=Link is up"]
pub type Rgmii1LinkIndicatorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RGMII1_SPEED` reader - 2:1\\]
Rgmii1 speed: 00=10Mbps, 01=100Mbps, 10=1000Mbps, 11=reserved"]
pub type Rgmii1SpeedR = crate::FieldReader;
#[doc = "Field `RGMII1_SPEED` writer - 2:1\\]
Rgmii1 speed: 00=10Mbps, 01=100Mbps, 10=1000Mbps, 11=reserved"]
pub type Rgmii1SpeedW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RGMII1_FULL_DULEX` reader - 3:3\\]
Rgmii1 full dulex: 0=Half-duplex, 1=Full-duplex"]
pub type Rgmii1FullDulexR = crate::BitReader;
#[doc = "Field `RGMII1_FULL_DULEX` writer - 3:3\\]
Rgmii1 full dulex: 0=Half-duplex, 1=Full-duplex"]
pub type Rgmii1FullDulexW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Rgmii1 link indicator: 0=Link is down, 1=Link is up"]
    #[inline(always)]
    pub fn rgmii1_link_indicator(&self) -> Rgmii1LinkIndicatorR {
        Rgmii1LinkIndicatorR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Rgmii1 speed: 00=10Mbps, 01=100Mbps, 10=1000Mbps, 11=reserved"]
    #[inline(always)]
    pub fn rgmii1_speed(&self) -> Rgmii1SpeedR {
        Rgmii1SpeedR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - 3:3\\]
Rgmii1 full dulex: 0=Half-duplex, 1=Full-duplex"]
    #[inline(always)]
    pub fn rgmii1_full_dulex(&self) -> Rgmii1FullDulexR {
        Rgmii1FullDulexR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Rgmii1 link indicator: 0=Link is down, 1=Link is up"]
    #[inline(always)]
    #[must_use]
    pub fn rgmii1_link_indicator(&mut self) -> Rgmii1LinkIndicatorW<Rgmii1StatusRegSpec> {
        Rgmii1LinkIndicatorW::new(self, 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Rgmii1 speed: 00=10Mbps, 01=100Mbps, 10=1000Mbps, 11=reserved"]
    #[inline(always)]
    #[must_use]
    pub fn rgmii1_speed(&mut self) -> Rgmii1SpeedW<Rgmii1StatusRegSpec> {
        Rgmii1SpeedW::new(self, 1)
    }
    #[doc = "Bit 3 - 3:3\\]
Rgmii1 full dulex: 0=Half-duplex, 1=Full-duplex"]
    #[inline(always)]
    #[must_use]
    pub fn rgmii1_full_dulex(&mut self) -> Rgmii1FullDulexW<Rgmii1StatusRegSpec> {
        Rgmii1FullDulexW::new(self, 3)
    }
}
#[doc = "RGMII1 Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rgmii1_status_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rgmii1_status_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rgmii1StatusRegSpec;
impl crate::RegisterSpec for Rgmii1StatusRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rgmii1_status_reg::R`](R) reader structure"]
impl crate::Readable for Rgmii1StatusRegSpec {}
#[doc = "`write(|w| ..)` method takes [`rgmii1_status_reg::W`](W) writer structure"]
impl crate::Writable for Rgmii1StatusRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RGMII1_STATUS_REG to value 0"]
impl crate::Resettable for Rgmii1StatusRegSpec {
    const RESET_VALUE: u32 = 0;
}
