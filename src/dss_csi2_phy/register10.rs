#[doc = "Register `REGISTER10` reader"]
pub type R = crate::R<Register10Spec>;
#[doc = "Register `REGISTER10` writer"]
pub type W = crate::W<Register10Spec>;
#[doc = "Field `EMPTY` reader - 3:0\\]
Reserved"]
pub type EmptyR = crate::FieldReader;
#[doc = "Field `EMPTY` writer - 3:0\\]
Reserved"]
pub type EmptyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `REGLDOEN` reader - 9:4\\]
1: Enable 0: Disable"]
pub type RegldoenR = crate::FieldReader;
#[doc = "Field `REGLDOEN` writer - 9:4\\]
1: Enable 0: Disable"]
pub type RegldoenW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `OVRRDLDOEN` reader - 10:10\\]
1: Override with Register bit 0: Default"]
pub type OvrrdldoenR = crate::BitReader;
#[doc = "Field `OVRRDLDOEN` writer - 10:10\\]
1: Override with Register bit 0: Default"]
pub type OvrrdldoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGHSTXSCPDAT4TO0` reader - 15:11\\]
15-11: Data for D4, D3, D2, D1 and D0 respectively"]
pub type Reghstxscpdat4to0R = crate::FieldReader;
#[doc = "Field `REGHSTXSCPDAT4TO0` writer - 15:11\\]
15-11: Data for D4, D3, D2, D1 and D0 respectively"]
pub type Reghstxscpdat4to0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ENHSTXSCPDAT` reader - 16:16\\]
1: HS Data Taken from register 0: Default"]
pub type EnhstxscpdatR = crate::BitReader;
#[doc = "Field `ENHSTXSCPDAT` writer - 16:16\\]
1: HS Data Taken from register 0: Default"]
pub type EnhstxscpdatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGLPTXSCPDAT4TO0DXDY` reader - 26:17\\]
26-17: Data for DX4,DY4,DX3,DY3, DX2,DY2, DX1,DY1, DX0,DY0 respectively"]
pub type Reglptxscpdat4to0dxdyR = crate::FieldReader<u16>;
#[doc = "Field `REGLPTXSCPDAT4TO0DXDY` writer - 26:17\\]
26-17: Data for DX4,DY4,DX3,DY3, DX2,DY2, DX1,DY1, DX0,DY0 respectively"]
pub type Reglptxscpdat4to0dxdyW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `ENLPTXSCPDAT` reader - 27:27\\]
1: LPTX Data Taken from register 0: Default"]
pub type EnlptxscpdatR = crate::BitReader;
#[doc = "Field `ENLPTXSCPDAT` writer - 27:27\\]
1: LPTX Data Taken from register 0: Default"]
pub type EnlptxscpdatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPPOWERUPTIME` reader - 29:28\\]
LDO wakeup time counter in number of PWRCLK cycles (approximate) 00: 12500 (Default) 01: 6250 10: 31250 11: 62500"]
pub type LppoweruptimeR = crate::FieldReader;
#[doc = "Field `LPPOWERUPTIME` writer - 29:28\\]
LDO wakeup time counter in number of PWRCLK cycles (approximate) 00: 12500 (Default) 01: 6250 10: 31250 11: 62500"]
pub type LppoweruptimeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LDOWAKEUPTIME` reader - 31:30\\]
Set LPTX wakeup time counter in PWRCLK cycles or TXCLKESC cycles. In PWRCLK cycles (when PWRCMD is changed). 00: 250 (Default) 01: 313 10: 375 11: 500"]
pub type LdowakeuptimeR = crate::FieldReader;
#[doc = "Field `LDOWAKEUPTIME` writer - 31:30\\]
Set LPTX wakeup time counter in PWRCLK cycles or TXCLKESC cycles. In PWRCLK cycles (when PWRCMD is changed). 00: 250 (Default) 01: 313 10: 375 11: 500"]
pub type LdowakeuptimeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Reserved"]
    #[inline(always)]
    pub fn empty(&self) -> EmptyR {
        EmptyR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:9 - 9:4\\]
1: Enable 0: Disable"]
    #[inline(always)]
    pub fn regldoen(&self) -> RegldoenR {
        RegldoenR::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bit 10 - 10:10\\]
1: Override with Register bit 0: Default"]
    #[inline(always)]
    pub fn ovrrdldoen(&self) -> OvrrdldoenR {
        OvrrdldoenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:15 - 15:11\\]
15-11: Data for D4, D3, D2, D1 and D0 respectively"]
    #[inline(always)]
    pub fn reghstxscpdat4to0(&self) -> Reghstxscpdat4to0R {
        Reghstxscpdat4to0R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
1: HS Data Taken from register 0: Default"]
    #[inline(always)]
    pub fn enhstxscpdat(&self) -> EnhstxscpdatR {
        EnhstxscpdatR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:26 - 26:17\\]
26-17: Data for DX4,DY4,DX3,DY3, DX2,DY2, DX1,DY1, DX0,DY0 respectively"]
    #[inline(always)]
    pub fn reglptxscpdat4to0dxdy(&self) -> Reglptxscpdat4to0dxdyR {
        Reglptxscpdat4to0dxdyR::new(((self.bits >> 17) & 0x03ff) as u16)
    }
    #[doc = "Bit 27 - 27:27\\]
1: LPTX Data Taken from register 0: Default"]
    #[inline(always)]
    pub fn enlptxscpdat(&self) -> EnlptxscpdatR {
        EnlptxscpdatR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - 29:28\\]
LDO wakeup time counter in number of PWRCLK cycles (approximate) 00: 12500 (Default) 01: 6250 10: 31250 11: 62500"]
    #[inline(always)]
    pub fn lppoweruptime(&self) -> LppoweruptimeR {
        LppoweruptimeR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Set LPTX wakeup time counter in PWRCLK cycles or TXCLKESC cycles. In PWRCLK cycles (when PWRCMD is changed). 00: 250 (Default) 01: 313 10: 375 11: 500"]
    #[inline(always)]
    pub fn ldowakeuptime(&self) -> LdowakeuptimeR {
        LdowakeuptimeR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn empty(&mut self) -> EmptyW<Register10Spec> {
        EmptyW::new(self, 0)
    }
    #[doc = "Bits 4:9 - 9:4\\]
1: Enable 0: Disable"]
    #[inline(always)]
    #[must_use]
    pub fn regldoen(&mut self) -> RegldoenW<Register10Spec> {
        RegldoenW::new(self, 4)
    }
    #[doc = "Bit 10 - 10:10\\]
1: Override with Register bit 0: Default"]
    #[inline(always)]
    #[must_use]
    pub fn ovrrdldoen(&mut self) -> OvrrdldoenW<Register10Spec> {
        OvrrdldoenW::new(self, 10)
    }
    #[doc = "Bits 11:15 - 15:11\\]
15-11: Data for D4, D3, D2, D1 and D0 respectively"]
    #[inline(always)]
    #[must_use]
    pub fn reghstxscpdat4to0(&mut self) -> Reghstxscpdat4to0W<Register10Spec> {
        Reghstxscpdat4to0W::new(self, 11)
    }
    #[doc = "Bit 16 - 16:16\\]
1: HS Data Taken from register 0: Default"]
    #[inline(always)]
    #[must_use]
    pub fn enhstxscpdat(&mut self) -> EnhstxscpdatW<Register10Spec> {
        EnhstxscpdatW::new(self, 16)
    }
    #[doc = "Bits 17:26 - 26:17\\]
26-17: Data for DX4,DY4,DX3,DY3, DX2,DY2, DX1,DY1, DX0,DY0 respectively"]
    #[inline(always)]
    #[must_use]
    pub fn reglptxscpdat4to0dxdy(&mut self) -> Reglptxscpdat4to0dxdyW<Register10Spec> {
        Reglptxscpdat4to0dxdyW::new(self, 17)
    }
    #[doc = "Bit 27 - 27:27\\]
1: LPTX Data Taken from register 0: Default"]
    #[inline(always)]
    #[must_use]
    pub fn enlptxscpdat(&mut self) -> EnlptxscpdatW<Register10Spec> {
        EnlptxscpdatW::new(self, 27)
    }
    #[doc = "Bits 28:29 - 29:28\\]
LDO wakeup time counter in number of PWRCLK cycles (approximate) 00: 12500 (Default) 01: 6250 10: 31250 11: 62500"]
    #[inline(always)]
    #[must_use]
    pub fn lppoweruptime(&mut self) -> LppoweruptimeW<Register10Spec> {
        LppoweruptimeW::new(self, 28)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Set LPTX wakeup time counter in PWRCLK cycles or TXCLKESC cycles. In PWRCLK cycles (when PWRCMD is changed). 00: 250 (Default) 01: 313 10: 375 11: 500"]
    #[inline(always)]
    #[must_use]
    pub fn ldowakeuptime(&mut self) -> LdowakeuptimeW<Register10Spec> {
        LdowakeuptimeW::new(self, 30)
    }
}
#[doc = "REGISTER10\n\nYou can [`read`](crate::Reg::read) this register and get [`register10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Register10Spec;
impl crate::RegisterSpec for Register10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register10::R`](R) reader structure"]
impl crate::Readable for Register10Spec {}
#[doc = "`write(|w| ..)` method takes [`register10::W`](W) writer structure"]
impl crate::Writable for Register10Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGISTER10 to value 0"]
impl crate::Resettable for Register10Spec {
    const RESET_VALUE: u32 = 0;
}
