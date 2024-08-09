#[doc = "Register `Programmable 6 MPPA` reader"]
pub type R = crate::R<Programmable6mppaSpec>;
#[doc = "Register `Programmable 6 MPPA` writer"]
pub type W = crate::W<Programmable6mppaSpec>;
#[doc = "Field `ux` reader - 0:0\\]
User executable permission. Defaults to input value."]
pub type UxR = crate::BitReader;
#[doc = "Field `ux` writer - 0:0\\]
User executable permission. Defaults to input value."]
pub type UxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uw` reader - 1:1\\]
User write permission. Defaults to input value."]
pub type UwR = crate::BitReader;
#[doc = "Field `uw` writer - 1:1\\]
User write permission. Defaults to input value."]
pub type UwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ur` reader - 2:2\\]
User read permission. Defaults to input value."]
pub type UrR = crate::BitReader;
#[doc = "Field `ur` writer - 2:2\\]
User read permission. Defaults to input value."]
pub type UrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sx` reader - 3:3\\]
Supervisor executable permission. Defaults to input value."]
pub type SxR = crate::BitReader;
#[doc = "Field `sx` writer - 3:3\\]
Supervisor executable permission. Defaults to input value."]
pub type SxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sw` reader - 4:4\\]
Supervisor write permission. Defaults to input value."]
pub type SwR = crate::BitReader;
#[doc = "Field `sw` writer - 4:4\\]
Supervisor write permission. Defaults to input value."]
pub type SwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sr` reader - 5:5\\]
Supervisor read permission. Defaults to input value."]
pub type SrR = crate::BitReader;
#[doc = "Field `sr` writer - 5:5\\]
Supervisor read permission. Defaults to input value."]
pub type SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `emu` reader - 6:6\\]
Debug permission. Defaults to input value."]
pub type EmuR = crate::BitReader;
#[doc = "Field `emu` writer - 6:6\\]
Debug permission. Defaults to input value."]
pub type EmuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ns` reader - 7:7\\]
Non-secure permission. Defaults to input value."]
pub type NsR = crate::BitReader;
#[doc = "Field `ns` writer - 7:7\\]
Non-secure permission. Defaults to input value."]
pub type NsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reserved1` reader - 8:8\\]
Always read as 0."]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `reserved1` writer - 8:8\\]
Always read as 0."]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AIDX` reader - 9:9\\]
Additional AIDs checked. Defaults to input value."]
pub type AidxR = crate::BitReader;
#[doc = "Field `AIDX` writer - 9:9\\]
Additional AIDs checked. Defaults to input value."]
pub type AidxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AID15_0` reader - 25:10\\]
AIDs checked for this region. Defaults to input value. 0 = AID is not checked for these permissions. 1 = AID is checked for these permissions."]
pub type Aid15_0R = crate::FieldReader<u16>;
#[doc = "Field `AID15_0` writer - 25:10\\]
AIDs checked for this region. Defaults to input value. 0 = AID is not checked for these permissions. 1 = AID is checked for these permissions."]
pub type Aid15_0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
User executable permission. Defaults to input value."]
    #[inline(always)]
    pub fn ux(&self) -> UxR {
        UxR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
User write permission. Defaults to input value."]
    #[inline(always)]
    pub fn uw(&self) -> UwR {
        UwR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
User read permission. Defaults to input value."]
    #[inline(always)]
    pub fn ur(&self) -> UrR {
        UrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Supervisor executable permission. Defaults to input value."]
    #[inline(always)]
    pub fn sx(&self) -> SxR {
        SxR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Supervisor write permission. Defaults to input value."]
    #[inline(always)]
    pub fn sw(&self) -> SwR {
        SwR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Supervisor read permission. Defaults to input value."]
    #[inline(always)]
    pub fn sr(&self) -> SrR {
        SrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Debug permission. Defaults to input value."]
    #[inline(always)]
    pub fn emu(&self) -> EmuR {
        EmuR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Non-secure permission. Defaults to input value."]
    #[inline(always)]
    pub fn ns(&self) -> NsR {
        NsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Always read as 0."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Additional AIDs checked. Defaults to input value."]
    #[inline(always)]
    pub fn aidx(&self) -> AidxR {
        AidxR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:25 - 25:10\\]
AIDs checked for this region. Defaults to input value. 0 = AID is not checked for these permissions. 1 = AID is checked for these permissions."]
    #[inline(always)]
    pub fn aid15_0(&self) -> Aid15_0R {
        Aid15_0R::new(((self.bits >> 10) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
User executable permission. Defaults to input value."]
    #[inline(always)]
    #[must_use]
    pub fn ux(&mut self) -> UxW<Programmable6mppaSpec> {
        UxW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
User write permission. Defaults to input value."]
    #[inline(always)]
    #[must_use]
    pub fn uw(&mut self) -> UwW<Programmable6mppaSpec> {
        UwW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
User read permission. Defaults to input value."]
    #[inline(always)]
    #[must_use]
    pub fn ur(&mut self) -> UrW<Programmable6mppaSpec> {
        UrW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Supervisor executable permission. Defaults to input value."]
    #[inline(always)]
    #[must_use]
    pub fn sx(&mut self) -> SxW<Programmable6mppaSpec> {
        SxW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Supervisor write permission. Defaults to input value."]
    #[inline(always)]
    #[must_use]
    pub fn sw(&mut self) -> SwW<Programmable6mppaSpec> {
        SwW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Supervisor read permission. Defaults to input value."]
    #[inline(always)]
    #[must_use]
    pub fn sr(&mut self) -> SrW<Programmable6mppaSpec> {
        SrW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Debug permission. Defaults to input value."]
    #[inline(always)]
    #[must_use]
    pub fn emu(&mut self) -> EmuW<Programmable6mppaSpec> {
        EmuW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Non-secure permission. Defaults to input value."]
    #[inline(always)]
    #[must_use]
    pub fn ns(&mut self) -> NsW<Programmable6mppaSpec> {
        NsW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Always read as 0."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<Programmable6mppaSpec> {
        Reserved1W::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Additional AIDs checked. Defaults to input value."]
    #[inline(always)]
    #[must_use]
    pub fn aidx(&mut self) -> AidxW<Programmable6mppaSpec> {
        AidxW::new(self, 9)
    }
    #[doc = "Bits 10:25 - 25:10\\]
AIDs checked for this region. Defaults to input value. 0 = AID is not checked for these permissions. 1 = AID is checked for these permissions."]
    #[inline(always)]
    #[must_use]
    pub fn aid15_0(&mut self) -> Aid15_0W<Programmable6mppaSpec> {
        Aid15_0W::new(self, 10)
    }
}
#[doc = "Programmable 6 MPPA\n\nYou can [`read`](crate::Reg::read) this register and get [`programmable6mppa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`programmable6mppa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Programmable6mppaSpec;
impl crate::RegisterSpec for Programmable6mppaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`programmable6mppa::R`](R) reader structure"]
impl crate::Readable for Programmable6mppaSpec {}
#[doc = "`write(|w| ..)` method takes [`programmable6mppa::W`](W) writer structure"]
impl crate::Writable for Programmable6mppaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets Programmable 6 MPPA to value 0"]
impl crate::Resettable for Programmable6mppaSpec {
    const RESET_VALUE: u32 = 0;
}
