#[doc = "Register `Fault Status` reader"]
pub type R = crate::R<FaultStatusSpec>;
#[doc = "Register `Fault Status` writer"]
pub type W = crate::W<FaultStatusSpec>;
#[doc = "Field `fault_type` reader - 5:0\\]
Fault type. 100000 = supervisor read fault 010000 = supervisor write fault 001000 = supervisor execute fault 000100 = user read fault 000010 = user write fault 000001 = user execute fault 111111 = relaxed cache linefill fault 010010 = relaxed cache writeback fault 000000 = no fault"]
pub type FaultTypeR = crate::FieldReader;
#[doc = "Field `fault_type` writer - 5:0\\]
Fault type. 100000 = supervisor read fault 010000 = supervisor write fault 001000 = supervisor execute fault 000100 = user read fault 000010 = user write fault 000001 = user execute fault 111111 = relaxed cache linefill fault 010010 = relaxed cache writeback fault 000000 = no fault"]
pub type FaultTypeW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `reserved2` reader - 6:6\\]
Always read as 0."]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `reserved2` writer - 6:6\\]
Always read as 0."]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ns` reader - 7:7\\]
Non-secure access."]
pub type NsR = crate::BitReader;
#[doc = "Field `ns` writer - 7:7\\]
Non-secure access."]
pub type NsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reserved1` reader - 8:8\\]
Always read as 0."]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `reserved1` writer - 8:8\\]
Always read as 0."]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `privid` reader - 12:9\\]
Privilege ID."]
pub type PrividR = crate::FieldReader;
#[doc = "Field `privid` writer - 12:9\\]
Privilege ID."]
pub type PrividW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `mstid` reader - 23:16\\]
Master ID."]
pub type MstidR = crate::FieldReader;
#[doc = "Field `mstid` writer - 23:16\\]
Master ID."]
pub type MstidW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `id` reader - 31:24\\]
Transfer ID"]
pub type IdR = crate::FieldReader;
#[doc = "Field `id` writer - 31:24\\]
Transfer ID"]
pub type IdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Fault type. 100000 = supervisor read fault 010000 = supervisor write fault 001000 = supervisor execute fault 000100 = user read fault 000010 = user write fault 000001 = user execute fault 111111 = relaxed cache linefill fault 010010 = relaxed cache writeback fault 000000 = no fault"]
    #[inline(always)]
    pub fn fault_type(&self) -> FaultTypeR {
        FaultTypeR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - 6:6\\]
Always read as 0."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Non-secure access."]
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
    #[doc = "Bits 9:12 - 12:9\\]
Privilege ID."]
    #[inline(always)]
    pub fn privid(&self) -> PrividR {
        PrividR::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Master ID."]
    #[inline(always)]
    pub fn mstid(&self) -> MstidR {
        MstidR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Transfer ID"]
    #[inline(always)]
    pub fn id(&self) -> IdR {
        IdR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Fault type. 100000 = supervisor read fault 010000 = supervisor write fault 001000 = supervisor execute fault 000100 = user read fault 000010 = user write fault 000001 = user execute fault 111111 = relaxed cache linefill fault 010010 = relaxed cache writeback fault 000000 = no fault"]
    #[inline(always)]
    #[must_use]
    pub fn fault_type(&mut self) -> FaultTypeW<FaultStatusSpec> {
        FaultTypeW::new(self, 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Always read as 0."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<FaultStatusSpec> {
        Reserved2W::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Non-secure access."]
    #[inline(always)]
    #[must_use]
    pub fn ns(&mut self) -> NsW<FaultStatusSpec> {
        NsW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Always read as 0."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<FaultStatusSpec> {
        Reserved1W::new(self, 8)
    }
    #[doc = "Bits 9:12 - 12:9\\]
Privilege ID."]
    #[inline(always)]
    #[must_use]
    pub fn privid(&mut self) -> PrividW<FaultStatusSpec> {
        PrividW::new(self, 9)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Master ID."]
    #[inline(always)]
    #[must_use]
    pub fn mstid(&mut self) -> MstidW<FaultStatusSpec> {
        MstidW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Transfer ID"]
    #[inline(always)]
    #[must_use]
    pub fn id(&mut self) -> IdW<FaultStatusSpec> {
        IdW::new(self, 24)
    }
}
#[doc = "Fault Status\n\nYou can [`read`](crate::Reg::read) this register and get [`fault_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fault_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FaultStatusSpec;
impl crate::RegisterSpec for FaultStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fault_status::R`](R) reader structure"]
impl crate::Readable for FaultStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`fault_status::W`](W) writer structure"]
impl crate::Writable for FaultStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets Fault Status to value 0"]
impl crate::Resettable for FaultStatusSpec {
    const RESET_VALUE: u32 = 0;
}
