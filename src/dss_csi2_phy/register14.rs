#[doc = "Register `REGISTER14` reader"]
pub type R = crate::R<Register14Spec>;
#[doc = "Register `REGISTER14` writer"]
pub type W = crate::W<Register14Spec>;
#[doc = "Field `EMPTY` reader - 5:0\\]
Reserved"]
pub type EmptyR = crate::FieldReader;
#[doc = "Field `EMPTY` writer - 5:0\\]
Reserved"]
pub type EmptyW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `REGHSTXDELAYSLAVECTRL` reader - 9:6\\]
Default: 0000"]
pub type ReghstxdelayslavectrlR = crate::FieldReader;
#[doc = "Field `REGHSTXDELAYSLAVECTRL` writer - 9:6\\]
Default: 0000"]
pub type ReghstxdelayslavectrlW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `OVRRDHSTXDELAYSLAVECTRL` reader - 10:10\\]
1: Override with register bit 0: Default"]
pub type OvrrdhstxdelayslavectrlR = crate::BitReader;
#[doc = "Field `OVRRDHSTXDELAYSLAVECTRL` writer - 10:10\\]
1: Override with register bit 0: Default"]
pub type OvrrdhstxdelayslavectrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRRDEFUSEBG` reader - 11:11\\]
1: Override EFUSE bits 0: Use EFUSE bits"]
pub type OvrrdefusebgR = crate::BitReader;
#[doc = "Field `OVRRDEFUSEBG` writer - 11:11\\]
1: Override EFUSE bits 0: Use EFUSE bits"]
pub type OvrrdefusebgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPRXSAMPLEOUTEN` reader - 12:12\\]
1: Enable sampled data to be brought out on WPO pins. 0: Normal mode"]
pub type LprxsampleoutenR = crate::BitReader;
#[doc = "Field `LPRXSAMPLEOUTEN` writer - 12:12\\]
1: Enable sampled data to be brought out on WPO pins. 0: Normal mode"]
pub type LprxsampleoutenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPCDSAMPLEOUTEN` reader - 13:13\\]
1: Enable sampled data to be brought out on WPO pins. 0: Normal mode"]
pub type LpcdsampleoutenR = crate::BitReader;
#[doc = "Field `LPCDSAMPLEOUTEN` writer - 13:13\\]
1: Enable sampled data to be brought out on WPO pins. 0: Normal mode"]
pub type LpcdsampleoutenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGHSTXDELAYMASTERCTRL` reader - 17:14\\]
Default: 0000"]
pub type ReghstxdelaymasterctrlR = crate::FieldReader;
#[doc = "Field `REGHSTXDELAYMASTERCTRL` writer - 17:14\\]
Default: 0000"]
pub type ReghstxdelaymasterctrlW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `OVRRDHSTXDELAYMASTERCTRL` reader - 18:18\\]
1: Override with register bit 0: Default"]
pub type OvrrdhstxdelaymasterctrlR = crate::BitReader;
#[doc = "Field `OVRRDHSTXDELAYMASTERCTRL` writer - 18:18\\]
1: Override with register bit 0: Default"]
pub type OvrrdhstxdelaymasterctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGHSDELAYCALIBCLRZ` reader - 19:19\\]
1: Clrz disabled(default) 0. Clrz enable"]
pub type ReghsdelaycalibclrzR = crate::BitReader;
#[doc = "Field `REGHSDELAYCALIBCLRZ` writer - 19:19\\]
1: Clrz disabled(default) 0. Clrz enable"]
pub type ReghsdelaycalibclrzW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRRDHSDELAYCALIBCLRZ` reader - 20:20\\]
1: Override with register bit 0: Default"]
pub type OvrrdhsdelaycalibclrzR = crate::BitReader;
#[doc = "Field `OVRRDHSDELAYCALIBCLRZ` writer - 20:20\\]
1: Override with register bit 0: Default"]
pub type OvrrdhsdelaycalibclrzW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGHSDELAYCALIBEN` reader - 21:21\\]
1: Enable 0: Disable"]
pub type ReghsdelaycalibenR = crate::BitReader;
#[doc = "Field `REGHSDELAYCALIBEN` writer - 21:21\\]
1: Enable 0: Disable"]
pub type ReghsdelaycalibenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRRDHSDELAYCALIBEN` reader - 22:22\\]
1: Override with register bit 0: Default"]
pub type OvrrdhsdelaycalibenR = crate::BitReader;
#[doc = "Field `OVRRDHSDELAYCALIBEN` writer - 22:22\\]
1: Override with register bit 0: Default"]
pub type OvrrdhsdelaycalibenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGBGCONTROL` reader - 30:23\\]
Default: 01111100"]
pub type RegbgcontrolR = crate::FieldReader;
#[doc = "Field `REGBGCONTROL` writer - 30:23\\]
Default: 01111100"]
pub type RegbgcontrolW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `OVRRDBGCONTROL` reader - 31:31\\]
1: Override bandgap control bits with register value 0: Default"]
pub type OvrrdbgcontrolR = crate::BitReader;
#[doc = "Field `OVRRDBGCONTROL` writer - 31:31\\]
1: Override bandgap control bits with register value 0: Default"]
pub type OvrrdbgcontrolW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Reserved"]
    #[inline(always)]
    pub fn empty(&self) -> EmptyR {
        EmptyR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:9 - 9:6\\]
Default: 0000"]
    #[inline(always)]
    pub fn reghstxdelayslavectrl(&self) -> ReghstxdelayslavectrlR {
        ReghstxdelayslavectrlR::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bit 10 - 10:10\\]
1: Override with register bit 0: Default"]
    #[inline(always)]
    pub fn ovrrdhstxdelayslavectrl(&self) -> OvrrdhstxdelayslavectrlR {
        OvrrdhstxdelayslavectrlR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
1: Override EFUSE bits 0: Use EFUSE bits"]
    #[inline(always)]
    pub fn ovrrdefusebg(&self) -> OvrrdefusebgR {
        OvrrdefusebgR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
1: Enable sampled data to be brought out on WPO pins. 0: Normal mode"]
    #[inline(always)]
    pub fn lprxsampleouten(&self) -> LprxsampleoutenR {
        LprxsampleoutenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
1: Enable sampled data to be brought out on WPO pins. 0: Normal mode"]
    #[inline(always)]
    pub fn lpcdsampleouten(&self) -> LpcdsampleoutenR {
        LpcdsampleoutenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:17 - 17:14\\]
Default: 0000"]
    #[inline(always)]
    pub fn reghstxdelaymasterctrl(&self) -> ReghstxdelaymasterctrlR {
        ReghstxdelaymasterctrlR::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bit 18 - 18:18\\]
1: Override with register bit 0: Default"]
    #[inline(always)]
    pub fn ovrrdhstxdelaymasterctrl(&self) -> OvrrdhstxdelaymasterctrlR {
        OvrrdhstxdelaymasterctrlR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
1: Clrz disabled(default) 0. Clrz enable"]
    #[inline(always)]
    pub fn reghsdelaycalibclrz(&self) -> ReghsdelaycalibclrzR {
        ReghsdelaycalibclrzR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
1: Override with register bit 0: Default"]
    #[inline(always)]
    pub fn ovrrdhsdelaycalibclrz(&self) -> OvrrdhsdelaycalibclrzR {
        OvrrdhsdelaycalibclrzR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
1: Enable 0: Disable"]
    #[inline(always)]
    pub fn reghsdelaycaliben(&self) -> ReghsdelaycalibenR {
        ReghsdelaycalibenR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
1: Override with register bit 0: Default"]
    #[inline(always)]
    pub fn ovrrdhsdelaycaliben(&self) -> OvrrdhsdelaycalibenR {
        OvrrdhsdelaycalibenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:30 - 30:23\\]
Default: 01111100"]
    #[inline(always)]
    pub fn regbgcontrol(&self) -> RegbgcontrolR {
        RegbgcontrolR::new(((self.bits >> 23) & 0xff) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
1: Override bandgap control bits with register value 0: Default"]
    #[inline(always)]
    pub fn ovrrdbgcontrol(&self) -> OvrrdbgcontrolR {
        OvrrdbgcontrolR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn empty(&mut self) -> EmptyW<Register14Spec> {
        EmptyW::new(self, 0)
    }
    #[doc = "Bits 6:9 - 9:6\\]
Default: 0000"]
    #[inline(always)]
    #[must_use]
    pub fn reghstxdelayslavectrl(&mut self) -> ReghstxdelayslavectrlW<Register14Spec> {
        ReghstxdelayslavectrlW::new(self, 6)
    }
    #[doc = "Bit 10 - 10:10\\]
1: Override with register bit 0: Default"]
    #[inline(always)]
    #[must_use]
    pub fn ovrrdhstxdelayslavectrl(&mut self) -> OvrrdhstxdelayslavectrlW<Register14Spec> {
        OvrrdhstxdelayslavectrlW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
1: Override EFUSE bits 0: Use EFUSE bits"]
    #[inline(always)]
    #[must_use]
    pub fn ovrrdefusebg(&mut self) -> OvrrdefusebgW<Register14Spec> {
        OvrrdefusebgW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
1: Enable sampled data to be brought out on WPO pins. 0: Normal mode"]
    #[inline(always)]
    #[must_use]
    pub fn lprxsampleouten(&mut self) -> LprxsampleoutenW<Register14Spec> {
        LprxsampleoutenW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
1: Enable sampled data to be brought out on WPO pins. 0: Normal mode"]
    #[inline(always)]
    #[must_use]
    pub fn lpcdsampleouten(&mut self) -> LpcdsampleoutenW<Register14Spec> {
        LpcdsampleoutenW::new(self, 13)
    }
    #[doc = "Bits 14:17 - 17:14\\]
Default: 0000"]
    #[inline(always)]
    #[must_use]
    pub fn reghstxdelaymasterctrl(&mut self) -> ReghstxdelaymasterctrlW<Register14Spec> {
        ReghstxdelaymasterctrlW::new(self, 14)
    }
    #[doc = "Bit 18 - 18:18\\]
1: Override with register bit 0: Default"]
    #[inline(always)]
    #[must_use]
    pub fn ovrrdhstxdelaymasterctrl(&mut self) -> OvrrdhstxdelaymasterctrlW<Register14Spec> {
        OvrrdhstxdelaymasterctrlW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
1: Clrz disabled(default) 0. Clrz enable"]
    #[inline(always)]
    #[must_use]
    pub fn reghsdelaycalibclrz(&mut self) -> ReghsdelaycalibclrzW<Register14Spec> {
        ReghsdelaycalibclrzW::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
1: Override with register bit 0: Default"]
    #[inline(always)]
    #[must_use]
    pub fn ovrrdhsdelaycalibclrz(&mut self) -> OvrrdhsdelaycalibclrzW<Register14Spec> {
        OvrrdhsdelaycalibclrzW::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
1: Enable 0: Disable"]
    #[inline(always)]
    #[must_use]
    pub fn reghsdelaycaliben(&mut self) -> ReghsdelaycalibenW<Register14Spec> {
        ReghsdelaycalibenW::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
1: Override with register bit 0: Default"]
    #[inline(always)]
    #[must_use]
    pub fn ovrrdhsdelaycaliben(&mut self) -> OvrrdhsdelaycalibenW<Register14Spec> {
        OvrrdhsdelaycalibenW::new(self, 22)
    }
    #[doc = "Bits 23:30 - 30:23\\]
Default: 01111100"]
    #[inline(always)]
    #[must_use]
    pub fn regbgcontrol(&mut self) -> RegbgcontrolW<Register14Spec> {
        RegbgcontrolW::new(self, 23)
    }
    #[doc = "Bit 31 - 31:31\\]
1: Override bandgap control bits with register value 0: Default"]
    #[inline(always)]
    #[must_use]
    pub fn ovrrdbgcontrol(&mut self) -> OvrrdbgcontrolW<Register14Spec> {
        OvrrdbgcontrolW::new(self, 31)
    }
}
#[doc = "REGISTER14\n\nYou can [`read`](crate::Reg::read) this register and get [`register14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Register14Spec;
impl crate::RegisterSpec for Register14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register14::R`](R) reader structure"]
impl crate::Readable for Register14Spec {}
#[doc = "`write(|w| ..)` method takes [`register14::W`](W) writer structure"]
impl crate::Writable for Register14Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGISTER14 to value 0"]
impl crate::Resettable for Register14Spec {
    const RESET_VALUE: u32 = 0;
}
