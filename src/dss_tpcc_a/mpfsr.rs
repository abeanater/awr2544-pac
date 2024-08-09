#[doc = "Register `MPFSR` reader"]
pub type R = crate::R<MpfsrSpec>;
#[doc = "Register `MPFSR` writer"]
pub type W = crate::W<MpfsrSpec>;
#[doc = "Field `UXE` reader - 0:0\\]
User Execute Error UXE = 0 : No error detected. UXE = 1 : User level task attempted to Execute from a MP Page without UX permissions."]
pub type UxeR = crate::BitReader;
#[doc = "Field `UXE` writer - 0:0\\]
User Execute Error UXE = 0 : No error detected. UXE = 1 : User level task attempted to Execute from a MP Page without UX permissions."]
pub type UxeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UWE` reader - 1:1\\]
User Write Error UWE = 0 : No error detected. UWE = 1 : User level task attempted to Write to a MP Page without UW permissions."]
pub type UweR = crate::BitReader;
#[doc = "Field `UWE` writer - 1:1\\]
User Write Error UWE = 0 : No error detected. UWE = 1 : User level task attempted to Write to a MP Page without UW permissions."]
pub type UweW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `URE` reader - 2:2\\]
User Read Error URE = 0 : No error detected. URE = 1 : User level task attempted to Read from a MP Page without UR permissions."]
pub type UreR = crate::BitReader;
#[doc = "Field `URE` writer - 2:2\\]
User Read Error URE = 0 : No error detected. URE = 1 : User level task attempted to Read from a MP Page without UR permissions."]
pub type UreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SXE` reader - 3:3\\]
Supervisor Execute Error SXE = 0 : No error detected. SXE = 1 : Supervisor level task attempted to Execute from a MP Page without SX permissions."]
pub type SxeR = crate::BitReader;
#[doc = "Field `SXE` writer - 3:3\\]
Supervisor Execute Error SXE = 0 : No error detected. SXE = 1 : Supervisor level task attempted to Execute from a MP Page without SX permissions."]
pub type SxeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWE` reader - 4:4\\]
Supervisor Write Error SWE = 0 : No error detected. SWE = 1 : Supervisor level task attempted to Write to a MP Page without SW permissions."]
pub type SweR = crate::BitReader;
#[doc = "Field `SWE` writer - 4:4\\]
Supervisor Write Error SWE = 0 : No error detected. SWE = 1 : Supervisor level task attempted to Write to a MP Page without SW permissions."]
pub type SweW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRE` reader - 5:5\\]
Supervisor Read Error SRE = 0 : No error detected. SRE = 1 : Supervisor level task attempted to Read from a MP Page without SR permissions."]
pub type SreR = crate::BitReader;
#[doc = "Field `SRE` writer - 5:5\\]
Supervisor Read Error SRE = 0 : No error detected. SRE = 1 : Supervisor level task attempted to Read from a MP Page without SR permissions."]
pub type SreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES0` reader - 8:6\\]
Reserved"]
pub type Res0R = crate::FieldReader;
#[doc = "Field `RES0` writer - 8:6\\]
Reserved"]
pub type Res0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FID` reader - 12:9\\]
Faulted ID FID register contains valid info if any of the MP error bits (UXE, UWE, URE, SXE, SWE, SRE) are non-zero (i.e., if an error has been detected.) The FID field contains the VBus PrivID for the specific request/requestor that resulted in a MP Error."]
pub type FidR = crate::FieldReader;
#[doc = "Field `FID` writer - 12:9\\]
Faulted ID FID register contains valid info if any of the MP error bits (UXE, UWE, URE, SXE, SWE, SRE) are non-zero (i.e., if an error has been detected.) The FID field contains the VBus PrivID for the specific request/requestor that resulted in a MP Error."]
pub type FidW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
User Execute Error UXE = 0 : No error detected. UXE = 1 : User level task attempted to Execute from a MP Page without UX permissions."]
    #[inline(always)]
    pub fn uxe(&self) -> UxeR {
        UxeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
User Write Error UWE = 0 : No error detected. UWE = 1 : User level task attempted to Write to a MP Page without UW permissions."]
    #[inline(always)]
    pub fn uwe(&self) -> UweR {
        UweR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
User Read Error URE = 0 : No error detected. URE = 1 : User level task attempted to Read from a MP Page without UR permissions."]
    #[inline(always)]
    pub fn ure(&self) -> UreR {
        UreR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Supervisor Execute Error SXE = 0 : No error detected. SXE = 1 : Supervisor level task attempted to Execute from a MP Page without SX permissions."]
    #[inline(always)]
    pub fn sxe(&self) -> SxeR {
        SxeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Supervisor Write Error SWE = 0 : No error detected. SWE = 1 : Supervisor level task attempted to Write to a MP Page without SW permissions."]
    #[inline(always)]
    pub fn swe(&self) -> SweR {
        SweR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Supervisor Read Error SRE = 0 : No error detected. SRE = 1 : Supervisor level task attempted to Read from a MP Page without SR permissions."]
    #[inline(always)]
    pub fn sre(&self) -> SreR {
        SreR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:8 - 8:6\\]
Reserved"]
    #[inline(always)]
    pub fn res0(&self) -> Res0R {
        Res0R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:12 - 12:9\\]
Faulted ID FID register contains valid info if any of the MP error bits (UXE, UWE, URE, SXE, SWE, SRE) are non-zero (i.e., if an error has been detected.) The FID field contains the VBus PrivID for the specific request/requestor that resulted in a MP Error."]
    #[inline(always)]
    pub fn fid(&self) -> FidR {
        FidR::new(((self.bits >> 9) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
User Execute Error UXE = 0 : No error detected. UXE = 1 : User level task attempted to Execute from a MP Page without UX permissions."]
    #[inline(always)]
    #[must_use]
    pub fn uxe(&mut self) -> UxeW<MpfsrSpec> {
        UxeW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
User Write Error UWE = 0 : No error detected. UWE = 1 : User level task attempted to Write to a MP Page without UW permissions."]
    #[inline(always)]
    #[must_use]
    pub fn uwe(&mut self) -> UweW<MpfsrSpec> {
        UweW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
User Read Error URE = 0 : No error detected. URE = 1 : User level task attempted to Read from a MP Page without UR permissions."]
    #[inline(always)]
    #[must_use]
    pub fn ure(&mut self) -> UreW<MpfsrSpec> {
        UreW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Supervisor Execute Error SXE = 0 : No error detected. SXE = 1 : Supervisor level task attempted to Execute from a MP Page without SX permissions."]
    #[inline(always)]
    #[must_use]
    pub fn sxe(&mut self) -> SxeW<MpfsrSpec> {
        SxeW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Supervisor Write Error SWE = 0 : No error detected. SWE = 1 : Supervisor level task attempted to Write to a MP Page without SW permissions."]
    #[inline(always)]
    #[must_use]
    pub fn swe(&mut self) -> SweW<MpfsrSpec> {
        SweW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Supervisor Read Error SRE = 0 : No error detected. SRE = 1 : Supervisor level task attempted to Read from a MP Page without SR permissions."]
    #[inline(always)]
    #[must_use]
    pub fn sre(&mut self) -> SreW<MpfsrSpec> {
        SreW::new(self, 5)
    }
    #[doc = "Bits 6:8 - 8:6\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn res0(&mut self) -> Res0W<MpfsrSpec> {
        Res0W::new(self, 6)
    }
    #[doc = "Bits 9:12 - 12:9\\]
Faulted ID FID register contains valid info if any of the MP error bits (UXE, UWE, URE, SXE, SWE, SRE) are non-zero (i.e., if an error has been detected.) The FID field contains the VBus PrivID for the specific request/requestor that resulted in a MP Error."]
    #[inline(always)]
    #[must_use]
    pub fn fid(&mut self) -> FidW<MpfsrSpec> {
        FidW::new(self, 9)
    }
}
#[doc = "Memory Protect Fault Status\n\nYou can [`read`](crate::Reg::read) this register and get [`mpfsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpfsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MpfsrSpec;
impl crate::RegisterSpec for MpfsrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`mpfsr::R`](R) reader structure"]
impl crate::Readable for MpfsrSpec {}
#[doc = "`write(|w| ..)` method takes [`mpfsr::W`](W) writer structure"]
impl crate::Writable for MpfsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets MPFSR to value 0"]
impl crate::Resettable for MpfsrSpec {
    const RESET_VALUE: u16 = 0;
}
