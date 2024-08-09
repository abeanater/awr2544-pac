#[doc = "Register `REGISTER9` reader"]
pub type R = crate::R<Register9Spec>;
#[doc = "Register `REGISTER9` writer"]
pub type W = crate::W<Register9Spec>;
#[doc = "Field `REGBYPASSACKZ` reader - 0:0\\]
BYPASSACKZ"]
pub type RegbypassackzR = crate::BitReader;
#[doc = "Field `REGBYPASSACKZ` writer - 0:0\\]
BYPASSACKZ"]
pub type RegbypassackzW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGCLKIN4DDRGODDBAR` reader - 1:1\\]
CLKIN4DDRGOODBAR"]
pub type Regclkin4ddrgoddbarR = crate::BitReader;
#[doc = "Field `REGCLKIN4DDRGODDBAR` writer - 1:1\\]
CLKIN4DDRGOODBAR"]
pub type Regclkin4ddrgoddbarW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGCLKINEN` reader - 2:2\\]
CLKINEN"]
pub type RegclkinenR = crate::BitReader;
#[doc = "Field `REGCLKINEN` writer - 2:2\\]
CLKINEN"]
pub type RegclkinenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGBYPASSEN` reader - 3:3\\]
BYPASSEN"]
pub type RegbypassenR = crate::BitReader;
#[doc = "Field `REGBYPASSEN` writer - 3:3\\]
BYPASSEN"]
pub type RegbypassenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRRDCLKIN4DDRSIGNALS` reader - 4:4\\]
1: Override signals with register settings 0: Default"]
pub type Ovrrdclkin4ddrsignalsR = crate::BitReader;
#[doc = "Field `OVRRDCLKIN4DDRSIGNALS` writer - 4:4\\]
1: Override signals with register settings 0: Default"]
pub type Ovrrdclkin4ddrsignalsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGLPTXIMPBYPASS` reader - 5:5\\]
1: output impedance bypassed permanently 0: output impedance not bypassed at all"]
pub type ReglptximpbypassR = crate::BitReader;
#[doc = "Field `REGLPTXIMPBYPASS` writer - 5:5\\]
1: output impedance bypassed permanently 0: output impedance not bypassed at all"]
pub type ReglptximpbypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENLPTXIMPBYPASS` reader - 6:6\\]
1: LP-TX bypass controlled by register bit. 0: Default value"]
pub type EnlptximpbypassR = crate::BitReader;
#[doc = "Field `ENLPTXIMPBYPASS` writer - 6:6\\]
1: LP-TX bypass controlled by register bit. 0: Default value"]
pub type EnlptximpbypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGVREGBIASCURR` reader - 8:7\\]
Default: 00"]
pub type RegvregbiascurrR = crate::FieldReader;
#[doc = "Field `REGVREGBIASCURR` writer - 8:7\\]
Default: 00"]
pub type RegvregbiascurrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REGVREGLOAD` reader - 10:9\\]
Default: 00"]
pub type RegvregloadR = crate::FieldReader;
#[doc = "Field `REGVREGLOAD` writer - 10:9\\]
Default: 00"]
pub type RegvregloadW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ENVREGCONTROL` reader - 11:11\\]
1: Enable control. 0: Use default current"]
pub type EnvregcontrolR = crate::BitReader;
#[doc = "Field `ENVREGCONTROL` writer - 11:11\\]
1: Enable control. 0: Use default current"]
pub type EnvregcontrolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGBIASGENTESTMODES` reader - 13:12\\]
Default: 00"]
pub type RegbiasgentestmodesR = crate::FieldReader;
#[doc = "Field `REGBIASGENTESTMODES` writer - 13:12\\]
Default: 00"]
pub type RegbiasgentestmodesW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ENBIASGENCONTROL` reader - 14:14\\]
1: Control Bias current through register bits. 0: Set bias current to default."]
pub type EnbiasgencontrolR = crate::BitReader;
#[doc = "Field `ENBIASGENCONTROL` writer - 14:14\\]
1: Control Bias current through register bits. 0: Set bias current to default."]
pub type EnbiasgencontrolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGLDOVOLTAGE` reader - 18:15\\]
1111: Maximum voltage 0000: Minimum voltage Default: 0100"]
pub type RegldovoltageR = crate::FieldReader;
#[doc = "Field `REGLDOVOLTAGE` writer - 18:15\\]
1111: Maximum voltage 0000: Minimum voltage Default: 0100"]
pub type RegldovoltageW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ENLDOVOLTAGECONTROL` reader - 19:19\\]
1: Control LDO output voltage through register bits. 0: Set LDO voltage to default"]
pub type EnldovoltagecontrolR = crate::BitReader;
#[doc = "Field `ENLDOVOLTAGECONTROL` writer - 19:19\\]
1: Control LDO output voltage through register bits. 0: Set LDO voltage to default"]
pub type EnldovoltagecontrolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENBIASGENCURROUT` reader - 20:20\\]
1: Enable biasgen current (10uA) brought out. 0: Biasgen current is not brought out.(Note this testmode will be used only in testchips. Not expected to be used in SOC)"]
pub type EnbiasgencurroutR = crate::BitReader;
#[doc = "Field `ENBIASGENCURROUT` writer - 20:20\\]
1: Enable biasgen current (10uA) brought out. 0: Biasgen current is not brought out.(Note this testmode will be used only in testchips. Not expected to be used in SOC)"]
pub type EnbiasgencurroutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGBANDGAPEN` reader - 21:21\\]
1: Enable 0: Disable"]
pub type RegbandgapenR = crate::BitReader;
#[doc = "Field `REGBANDGAPEN` writer - 21:21\\]
1: Enable 0: Disable"]
pub type RegbandgapenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRRDBANDGAPEN` reader - 22:22\\]
1: Override with Register bit 0: Default"]
pub type OvrrdbandgapenR = crate::BitReader;
#[doc = "Field `OVRRDBANDGAPEN` writer - 22:22\\]
1: Override with Register bit 0: Default"]
pub type OvrrdbandgapenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGBIASGENEN` reader - 23:23\\]
1: Enable 0: Disable"]
pub type RegbiasgenenR = crate::BitReader;
#[doc = "Field `REGBIASGENEN` writer - 23:23\\]
1: Enable 0: Disable"]
pub type RegbiasgenenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRRDBIASGENEN` reader - 24:24\\]
1: Override with Register bit 0 : Default"]
pub type OvrrdbiasgenenR = crate::BitReader;
#[doc = "Field `OVRRDBIASGENEN` writer - 24:24\\]
1: Override with Register bit 0 : Default"]
pub type OvrrdbiasgenenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMPTY` reader - 25:25\\]
Reserved"]
pub type EmptyR = crate::BitReader;
#[doc = "Field `EMPTY` writer - 25:25\\]
Reserved"]
pub type EmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGPOLARITY3TO0` reader - 30:26\\]
1: DX=DN, DY=DP 0: DX=DP, DY=DN"]
pub type Regpolarity3to0R = crate::FieldReader;
#[doc = "Field `REGPOLARITY3TO0` writer - 30:26\\]
1: DX=DN, DY=DP 0: DX=DP, DY=DN"]
pub type Regpolarity3to0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OVRRDPOLARITY` reader - 31:31\\]
1: Override with register bit 0: Default"]
pub type OvrrdpolarityR = crate::BitReader;
#[doc = "Field `OVRRDPOLARITY` writer - 31:31\\]
1: Override with register bit 0: Default"]
pub type OvrrdpolarityW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
BYPASSACKZ"]
    #[inline(always)]
    pub fn regbypassackz(&self) -> RegbypassackzR {
        RegbypassackzR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
CLKIN4DDRGOODBAR"]
    #[inline(always)]
    pub fn regclkin4ddrgoddbar(&self) -> Regclkin4ddrgoddbarR {
        Regclkin4ddrgoddbarR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
CLKINEN"]
    #[inline(always)]
    pub fn regclkinen(&self) -> RegclkinenR {
        RegclkinenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
BYPASSEN"]
    #[inline(always)]
    pub fn regbypassen(&self) -> RegbypassenR {
        RegbypassenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
1: Override signals with register settings 0: Default"]
    #[inline(always)]
    pub fn ovrrdclkin4ddrsignals(&self) -> Ovrrdclkin4ddrsignalsR {
        Ovrrdclkin4ddrsignalsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
1: output impedance bypassed permanently 0: output impedance not bypassed at all"]
    #[inline(always)]
    pub fn reglptximpbypass(&self) -> ReglptximpbypassR {
        ReglptximpbypassR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
1: LP-TX bypass controlled by register bit. 0: Default value"]
    #[inline(always)]
    pub fn enlptximpbypass(&self) -> EnlptximpbypassR {
        EnlptximpbypassR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - 8:7\\]
Default: 00"]
    #[inline(always)]
    pub fn regvregbiascurr(&self) -> RegvregbiascurrR {
        RegvregbiascurrR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:10 - 10:9\\]
Default: 00"]
    #[inline(always)]
    pub fn regvregload(&self) -> RegvregloadR {
        RegvregloadR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - 11:11\\]
1: Enable control. 0: Use default current"]
    #[inline(always)]
    pub fn envregcontrol(&self) -> EnvregcontrolR {
        EnvregcontrolR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Default: 00"]
    #[inline(always)]
    pub fn regbiasgentestmodes(&self) -> RegbiasgentestmodesR {
        RegbiasgentestmodesR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
1: Control Bias current through register bits. 0: Set bias current to default."]
    #[inline(always)]
    pub fn enbiasgencontrol(&self) -> EnbiasgencontrolR {
        EnbiasgencontrolR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:18 - 18:15\\]
1111: Maximum voltage 0000: Minimum voltage Default: 0100"]
    #[inline(always)]
    pub fn regldovoltage(&self) -> RegldovoltageR {
        RegldovoltageR::new(((self.bits >> 15) & 0x0f) as u8)
    }
    #[doc = "Bit 19 - 19:19\\]
1: Control LDO output voltage through register bits. 0: Set LDO voltage to default"]
    #[inline(always)]
    pub fn enldovoltagecontrol(&self) -> EnldovoltagecontrolR {
        EnldovoltagecontrolR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
1: Enable biasgen current (10uA) brought out. 0: Biasgen current is not brought out.(Note this testmode will be used only in testchips. Not expected to be used in SOC)"]
    #[inline(always)]
    pub fn enbiasgencurrout(&self) -> EnbiasgencurroutR {
        EnbiasgencurroutR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
1: Enable 0: Disable"]
    #[inline(always)]
    pub fn regbandgapen(&self) -> RegbandgapenR {
        RegbandgapenR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
1: Override with Register bit 0: Default"]
    #[inline(always)]
    pub fn ovrrdbandgapen(&self) -> OvrrdbandgapenR {
        OvrrdbandgapenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
1: Enable 0: Disable"]
    #[inline(always)]
    pub fn regbiasgenen(&self) -> RegbiasgenenR {
        RegbiasgenenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
1: Override with Register bit 0 : Default"]
    #[inline(always)]
    pub fn ovrrdbiasgenen(&self) -> OvrrdbiasgenenR {
        OvrrdbiasgenenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Reserved"]
    #[inline(always)]
    pub fn empty(&self) -> EmptyR {
        EmptyR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:30 - 30:26\\]
1: DX=DN, DY=DP 0: DX=DP, DY=DN"]
    #[inline(always)]
    pub fn regpolarity3to0(&self) -> Regpolarity3to0R {
        Regpolarity3to0R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
1: Override with register bit 0: Default"]
    #[inline(always)]
    pub fn ovrrdpolarity(&self) -> OvrrdpolarityR {
        OvrrdpolarityR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
BYPASSACKZ"]
    #[inline(always)]
    #[must_use]
    pub fn regbypassackz(&mut self) -> RegbypassackzW<Register9Spec> {
        RegbypassackzW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
CLKIN4DDRGOODBAR"]
    #[inline(always)]
    #[must_use]
    pub fn regclkin4ddrgoddbar(&mut self) -> Regclkin4ddrgoddbarW<Register9Spec> {
        Regclkin4ddrgoddbarW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
CLKINEN"]
    #[inline(always)]
    #[must_use]
    pub fn regclkinen(&mut self) -> RegclkinenW<Register9Spec> {
        RegclkinenW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
BYPASSEN"]
    #[inline(always)]
    #[must_use]
    pub fn regbypassen(&mut self) -> RegbypassenW<Register9Spec> {
        RegbypassenW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
1: Override signals with register settings 0: Default"]
    #[inline(always)]
    #[must_use]
    pub fn ovrrdclkin4ddrsignals(&mut self) -> Ovrrdclkin4ddrsignalsW<Register9Spec> {
        Ovrrdclkin4ddrsignalsW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
1: output impedance bypassed permanently 0: output impedance not bypassed at all"]
    #[inline(always)]
    #[must_use]
    pub fn reglptximpbypass(&mut self) -> ReglptximpbypassW<Register9Spec> {
        ReglptximpbypassW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
1: LP-TX bypass controlled by register bit. 0: Default value"]
    #[inline(always)]
    #[must_use]
    pub fn enlptximpbypass(&mut self) -> EnlptximpbypassW<Register9Spec> {
        EnlptximpbypassW::new(self, 6)
    }
    #[doc = "Bits 7:8 - 8:7\\]
Default: 00"]
    #[inline(always)]
    #[must_use]
    pub fn regvregbiascurr(&mut self) -> RegvregbiascurrW<Register9Spec> {
        RegvregbiascurrW::new(self, 7)
    }
    #[doc = "Bits 9:10 - 10:9\\]
Default: 00"]
    #[inline(always)]
    #[must_use]
    pub fn regvregload(&mut self) -> RegvregloadW<Register9Spec> {
        RegvregloadW::new(self, 9)
    }
    #[doc = "Bit 11 - 11:11\\]
1: Enable control. 0: Use default current"]
    #[inline(always)]
    #[must_use]
    pub fn envregcontrol(&mut self) -> EnvregcontrolW<Register9Spec> {
        EnvregcontrolW::new(self, 11)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Default: 00"]
    #[inline(always)]
    #[must_use]
    pub fn regbiasgentestmodes(&mut self) -> RegbiasgentestmodesW<Register9Spec> {
        RegbiasgentestmodesW::new(self, 12)
    }
    #[doc = "Bit 14 - 14:14\\]
1: Control Bias current through register bits. 0: Set bias current to default."]
    #[inline(always)]
    #[must_use]
    pub fn enbiasgencontrol(&mut self) -> EnbiasgencontrolW<Register9Spec> {
        EnbiasgencontrolW::new(self, 14)
    }
    #[doc = "Bits 15:18 - 18:15\\]
1111: Maximum voltage 0000: Minimum voltage Default: 0100"]
    #[inline(always)]
    #[must_use]
    pub fn regldovoltage(&mut self) -> RegldovoltageW<Register9Spec> {
        RegldovoltageW::new(self, 15)
    }
    #[doc = "Bit 19 - 19:19\\]
1: Control LDO output voltage through register bits. 0: Set LDO voltage to default"]
    #[inline(always)]
    #[must_use]
    pub fn enldovoltagecontrol(&mut self) -> EnldovoltagecontrolW<Register9Spec> {
        EnldovoltagecontrolW::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
1: Enable biasgen current (10uA) brought out. 0: Biasgen current is not brought out.(Note this testmode will be used only in testchips. Not expected to be used in SOC)"]
    #[inline(always)]
    #[must_use]
    pub fn enbiasgencurrout(&mut self) -> EnbiasgencurroutW<Register9Spec> {
        EnbiasgencurroutW::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
1: Enable 0: Disable"]
    #[inline(always)]
    #[must_use]
    pub fn regbandgapen(&mut self) -> RegbandgapenW<Register9Spec> {
        RegbandgapenW::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
1: Override with Register bit 0: Default"]
    #[inline(always)]
    #[must_use]
    pub fn ovrrdbandgapen(&mut self) -> OvrrdbandgapenW<Register9Spec> {
        OvrrdbandgapenW::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
1: Enable 0: Disable"]
    #[inline(always)]
    #[must_use]
    pub fn regbiasgenen(&mut self) -> RegbiasgenenW<Register9Spec> {
        RegbiasgenenW::new(self, 23)
    }
    #[doc = "Bit 24 - 24:24\\]
1: Override with Register bit 0 : Default"]
    #[inline(always)]
    #[must_use]
    pub fn ovrrdbiasgenen(&mut self) -> OvrrdbiasgenenW<Register9Spec> {
        OvrrdbiasgenenW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn empty(&mut self) -> EmptyW<Register9Spec> {
        EmptyW::new(self, 25)
    }
    #[doc = "Bits 26:30 - 30:26\\]
1: DX=DN, DY=DP 0: DX=DP, DY=DN"]
    #[inline(always)]
    #[must_use]
    pub fn regpolarity3to0(&mut self) -> Regpolarity3to0W<Register9Spec> {
        Regpolarity3to0W::new(self, 26)
    }
    #[doc = "Bit 31 - 31:31\\]
1: Override with register bit 0: Default"]
    #[inline(always)]
    #[must_use]
    pub fn ovrrdpolarity(&mut self) -> OvrrdpolarityW<Register9Spec> {
        OvrrdpolarityW::new(self, 31)
    }
}
#[doc = "REGISTER9\n\nYou can [`read`](crate::Reg::read) this register and get [`register9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Register9Spec;
impl crate::RegisterSpec for Register9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register9::R`](R) reader structure"]
impl crate::Readable for Register9Spec {}
#[doc = "`write(|w| ..)` method takes [`register9::W`](W) writer structure"]
impl crate::Writable for Register9Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGISTER9 to value 0"]
impl crate::Resettable for Register9Spec {
    const RESET_VALUE: u32 = 0;
}
