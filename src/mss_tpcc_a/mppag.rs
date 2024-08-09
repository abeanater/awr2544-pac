#[doc = "Register `MPPAG` reader"]
pub type R = crate::R<MppagSpec>;
#[doc = "Register `MPPAG` writer"]
pub type W = crate::W<MppagSpec>;
#[doc = "Field `UX` reader - 0:0\\]
User Execute permission UX = 0 : User execute accesses are not allowed from Region N.. UX = 1 : User execute accesses are allowed from Region N addresses."]
pub type UxR = crate::BitReader;
#[doc = "Field `UX` writer - 0:0\\]
User Execute permission UX = 0 : User execute accesses are not allowed from Region N.. UX = 1 : User execute accesses are allowed from Region N addresses."]
pub type UxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UW` reader - 1:1\\]
User Write permission UW = 0 : User write accesses are not allowed to Region N.. UW = 1 : User write accesses are allowed to Region N addresses."]
pub type UwR = crate::BitReader;
#[doc = "Field `UW` writer - 1:1\\]
User Write permission UW = 0 : User write accesses are not allowed to Region N.. UW = 1 : User write accesses are allowed to Region N addresses."]
pub type UwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UR` reader - 2:2\\]
User Read permission UR = 0 : User read accesses are not allowed from Region N.. UR = 1 : User write accesses are allowed from Region N addresses"]
pub type UrR = crate::BitReader;
#[doc = "Field `UR` writer - 2:2\\]
User Read permission UR = 0 : User read accesses are not allowed from Region N.. UR = 1 : User write accesses are allowed from Region N addresses"]
pub type UrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SX` reader - 3:3\\]
Supervisor Execute permission SX = 0 : Supervisor execute accesses are not allowed from Region N.. SX = 1 : Supervisor execute accesses are allowed from Region N addresses"]
pub type SxR = crate::BitReader;
#[doc = "Field `SX` writer - 3:3\\]
Supervisor Execute permission SX = 0 : Supervisor execute accesses are not allowed from Region N.. SX = 1 : Supervisor execute accesses are allowed from Region N addresses"]
pub type SxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW` reader - 4:4\\]
Supervisor Write permission SW = 0 : Supervisor write accesses are not allowed to Region N.. SW = 1 : Supervisor write accesses are allowed to Region N addresses."]
pub type SwR = crate::BitReader;
#[doc = "Field `SW` writer - 4:4\\]
Supervisor Write permission SW = 0 : Supervisor write accesses are not allowed to Region N.. SW = 1 : Supervisor write accesses are allowed to Region N addresses."]
pub type SwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SR` reader - 5:5\\]
Supervisor Read permission SR = 0 : Supervisor read accesses are not allowed from Region N.. SR = 1 : Supervisor write accesses are allowed from Region N addresses"]
pub type SrR = crate::BitReader;
#[doc = "Field `SR` writer - 5:5\\]
Supervisor Read permission SR = 0 : Supervisor read accesses are not allowed from Region N.. SR = 1 : Supervisor write accesses are allowed from Region N addresses"]
pub type SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMU` reader - 6:6\\]
Emulation Security permission EMU = 0 : Emulation (cfg_emudbg == 1) reads/writes to this page are NOT permitted if the page is marked secure (NS = 0). EMU = 1 : Emulation reads/writes to this page ARE permitted."]
pub type EmuR = crate::BitReader;
#[doc = "Field `EMU` writer - 6:6\\]
Emulation Security permission EMU = 0 : Emulation (cfg_emudbg == 1) reads/writes to this page are NOT permitted if the page is marked secure (NS = 0). EMU = 1 : Emulation reads/writes to this page ARE permitted."]
pub type EmuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NS` reader - 7:7\\]
Secure access permission NS = 0 : Page is secure. Only Secure mode non-emulation accesses may access this page. Emulation accessibility is controlled by EMU setting. NS = 1 : Page is not Secure. Both Secure and Non-secure code may access this page"]
pub type NsR = crate::BitReader;
#[doc = "Field `NS` writer - 7:7\\]
Secure access permission NS = 0 : Page is secure. Only Secure mode non-emulation accesses may access this page. Emulation accessibility is controlled by EMU setting. NS = 1 : Page is not Secure. Both Secure and Non-secure code may access this page"]
pub type NsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES0` reader - 8:8\\]
Local accesses (i.e., Link Address updates) are always allowed. LCL bit is not used to allow/disallow Link address updates."]
pub type Res0R = crate::BitReader;
#[doc = "Field `RES0` writer - 8:8\\]
Local accesses (i.e., Link Address updates) are always allowed. LCL bit is not used to allow/disallow Link address updates."]
pub type Res0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXT` reader - 9:9\\]
External Allowed ID AIDm = 0 : VBus requests with PrivID >= ΓÇÿ6ΓÇÖ are not allowed to Region N regardless of permission settings (UW, UR, SW, SR). AIDm = 1 : VBus requests with PrivID >= ΓÇÿ6ΓÇÖ are permitted if access type is allowed as defined by permission settings (UW, UR, SW, SR)."]
pub type ExtR = crate::BitReader;
#[doc = "Field `EXT` writer - 9:9\\]
External Allowed ID AIDm = 0 : VBus requests with PrivID >= ΓÇÿ6ΓÇÖ are not allowed to Region N regardless of permission settings (UW, UR, SW, SR). AIDm = 1 : VBus requests with PrivID >= ΓÇÿ6ΓÇÖ are permitted if access type is allowed as defined by permission settings (UW, UR, SW, SR)."]
pub type ExtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AID` reader - 15:10\\]
Allowed ID ΓÇÿMΓÇÖ AIDm = 0 : VBus requests with PrivID == ΓÇÿMΓÇÖ are not allowed to Region N regardless of permission settings (UW, UR, SW, SR). AIDm = 1 : VBus requests with PrivID == ΓÇÿMΓÇÖ are permitted if access type is allowed as defined by permission settings (UW, UR, SW, SR)."]
pub type AidR = crate::FieldReader;
#[doc = "Field `AID` writer - 15:10\\]
Allowed ID ΓÇÿMΓÇÖ AIDm = 0 : VBus requests with PrivID == ΓÇÿMΓÇÖ are not allowed to Region N regardless of permission settings (UW, UR, SW, SR). AIDm = 1 : VBus requests with PrivID == ΓÇÿMΓÇÖ are permitted if access type is allowed as defined by permission settings (UW, UR, SW, SR)."]
pub type AidW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RES1` reader - 31:16\\]
Reserved"]
pub type Res1R = crate::FieldReader<u16>;
#[doc = "Field `RES1` writer - 31:16\\]
Reserved"]
pub type Res1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
User Execute permission UX = 0 : User execute accesses are not allowed from Region N.. UX = 1 : User execute accesses are allowed from Region N addresses."]
    #[inline(always)]
    pub fn ux(&self) -> UxR {
        UxR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
User Write permission UW = 0 : User write accesses are not allowed to Region N.. UW = 1 : User write accesses are allowed to Region N addresses."]
    #[inline(always)]
    pub fn uw(&self) -> UwR {
        UwR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
User Read permission UR = 0 : User read accesses are not allowed from Region N.. UR = 1 : User write accesses are allowed from Region N addresses"]
    #[inline(always)]
    pub fn ur(&self) -> UrR {
        UrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Supervisor Execute permission SX = 0 : Supervisor execute accesses are not allowed from Region N.. SX = 1 : Supervisor execute accesses are allowed from Region N addresses"]
    #[inline(always)]
    pub fn sx(&self) -> SxR {
        SxR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Supervisor Write permission SW = 0 : Supervisor write accesses are not allowed to Region N.. SW = 1 : Supervisor write accesses are allowed to Region N addresses."]
    #[inline(always)]
    pub fn sw(&self) -> SwR {
        SwR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Supervisor Read permission SR = 0 : Supervisor read accesses are not allowed from Region N.. SR = 1 : Supervisor write accesses are allowed from Region N addresses"]
    #[inline(always)]
    pub fn sr(&self) -> SrR {
        SrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Emulation Security permission EMU = 0 : Emulation (cfg_emudbg == 1) reads/writes to this page are NOT permitted if the page is marked secure (NS = 0). EMU = 1 : Emulation reads/writes to this page ARE permitted."]
    #[inline(always)]
    pub fn emu(&self) -> EmuR {
        EmuR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Secure access permission NS = 0 : Page is secure. Only Secure mode non-emulation accesses may access this page. Emulation accessibility is controlled by EMU setting. NS = 1 : Page is not Secure. Both Secure and Non-secure code may access this page"]
    #[inline(always)]
    pub fn ns(&self) -> NsR {
        NsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Local accesses (i.e., Link Address updates) are always allowed. LCL bit is not used to allow/disallow Link address updates."]
    #[inline(always)]
    pub fn res0(&self) -> Res0R {
        Res0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
External Allowed ID AIDm = 0 : VBus requests with PrivID >= ΓÇÿ6ΓÇÖ are not allowed to Region N regardless of permission settings (UW, UR, SW, SR). AIDm = 1 : VBus requests with PrivID >= ΓÇÿ6ΓÇÖ are permitted if access type is allowed as defined by permission settings (UW, UR, SW, SR)."]
    #[inline(always)]
    pub fn ext(&self) -> ExtR {
        ExtR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - 15:10\\]
Allowed ID ΓÇÿMΓÇÖ AIDm = 0 : VBus requests with PrivID == ΓÇÿMΓÇÖ are not allowed to Region N regardless of permission settings (UW, UR, SW, SR). AIDm = 1 : VBus requests with PrivID == ΓÇÿMΓÇÖ are permitted if access type is allowed as defined by permission settings (UW, UR, SW, SR)."]
    #[inline(always)]
    pub fn aid(&self) -> AidR {
        AidR::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved"]
    #[inline(always)]
    pub fn res1(&self) -> Res1R {
        Res1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
User Execute permission UX = 0 : User execute accesses are not allowed from Region N.. UX = 1 : User execute accesses are allowed from Region N addresses."]
    #[inline(always)]
    #[must_use]
    pub fn ux(&mut self) -> UxW<MppagSpec> {
        UxW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
User Write permission UW = 0 : User write accesses are not allowed to Region N.. UW = 1 : User write accesses are allowed to Region N addresses."]
    #[inline(always)]
    #[must_use]
    pub fn uw(&mut self) -> UwW<MppagSpec> {
        UwW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
User Read permission UR = 0 : User read accesses are not allowed from Region N.. UR = 1 : User write accesses are allowed from Region N addresses"]
    #[inline(always)]
    #[must_use]
    pub fn ur(&mut self) -> UrW<MppagSpec> {
        UrW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Supervisor Execute permission SX = 0 : Supervisor execute accesses are not allowed from Region N.. SX = 1 : Supervisor execute accesses are allowed from Region N addresses"]
    #[inline(always)]
    #[must_use]
    pub fn sx(&mut self) -> SxW<MppagSpec> {
        SxW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Supervisor Write permission SW = 0 : Supervisor write accesses are not allowed to Region N.. SW = 1 : Supervisor write accesses are allowed to Region N addresses."]
    #[inline(always)]
    #[must_use]
    pub fn sw(&mut self) -> SwW<MppagSpec> {
        SwW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Supervisor Read permission SR = 0 : Supervisor read accesses are not allowed from Region N.. SR = 1 : Supervisor write accesses are allowed from Region N addresses"]
    #[inline(always)]
    #[must_use]
    pub fn sr(&mut self) -> SrW<MppagSpec> {
        SrW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Emulation Security permission EMU = 0 : Emulation (cfg_emudbg == 1) reads/writes to this page are NOT permitted if the page is marked secure (NS = 0). EMU = 1 : Emulation reads/writes to this page ARE permitted."]
    #[inline(always)]
    #[must_use]
    pub fn emu(&mut self) -> EmuW<MppagSpec> {
        EmuW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Secure access permission NS = 0 : Page is secure. Only Secure mode non-emulation accesses may access this page. Emulation accessibility is controlled by EMU setting. NS = 1 : Page is not Secure. Both Secure and Non-secure code may access this page"]
    #[inline(always)]
    #[must_use]
    pub fn ns(&mut self) -> NsW<MppagSpec> {
        NsW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Local accesses (i.e., Link Address updates) are always allowed. LCL bit is not used to allow/disallow Link address updates."]
    #[inline(always)]
    #[must_use]
    pub fn res0(&mut self) -> Res0W<MppagSpec> {
        Res0W::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
External Allowed ID AIDm = 0 : VBus requests with PrivID >= ΓÇÿ6ΓÇÖ are not allowed to Region N regardless of permission settings (UW, UR, SW, SR). AIDm = 1 : VBus requests with PrivID >= ΓÇÿ6ΓÇÖ are permitted if access type is allowed as defined by permission settings (UW, UR, SW, SR)."]
    #[inline(always)]
    #[must_use]
    pub fn ext(&mut self) -> ExtW<MppagSpec> {
        ExtW::new(self, 9)
    }
    #[doc = "Bits 10:15 - 15:10\\]
Allowed ID ΓÇÿMΓÇÖ AIDm = 0 : VBus requests with PrivID == ΓÇÿMΓÇÖ are not allowed to Region N regardless of permission settings (UW, UR, SW, SR). AIDm = 1 : VBus requests with PrivID == ΓÇÿMΓÇÖ are permitted if access type is allowed as defined by permission settings (UW, UR, SW, SR)."]
    #[inline(always)]
    #[must_use]
    pub fn aid(&mut self) -> AidW<MppagSpec> {
        AidW::new(self, 10)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn res1(&mut self) -> Res1W<MppagSpec> {
        Res1W::new(self, 16)
    }
}
#[doc = "MP Permission Attribute for Global Regs\n\nYou can [`read`](crate::Reg::read) this register and get [`mppag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mppag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MppagSpec;
impl crate::RegisterSpec for MppagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mppag::R`](R) reader structure"]
impl crate::Readable for MppagSpec {}
#[doc = "`write(|w| ..)` method takes [`mppag::W`](W) writer structure"]
impl crate::Writable for MppagSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MPPAG to value 0"]
impl crate::Resettable for MppagSpec {
    const RESET_VALUE: u32 = 0;
}
