#[doc = "Register `PPCS0MSTID` reader"]
pub type R = crate::R<Ppcs0mstidSpec>;
#[doc = "Register `PPCS0MSTID` writer"]
pub type W = crate::W<Ppcs0mstidSpec>;
#[doc = "Field `PPCS0MSTID` reader - 15:0\\]
There are 16 bits for each frame in PPCS. These bits sets the permission for maximum of 16 masters to address the memory mapped in each of the frame. The scheme is similar to the one described for PCSm MSTID in section 1.7.33. Readable in both user and privileged modes. 1 = The memory mapped in respective frames can be addressed by master with matching Master-ID. 0 = The memory is locked for master with matching Master-ID. PCR responds with AERROR. Writable only in privileged mode 1 = Sets the corresponding bit. 0 = Clears the corresponding bit. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Ppcs0mstidR = crate::FieldReader<u16>;
#[doc = "Field `PPCS0MSTID` writer - 15:0\\]
There are 16 bits for each frame in PPCS. These bits sets the permission for maximum of 16 masters to address the memory mapped in each of the frame. The scheme is similar to the one described for PCSm MSTID in section 1.7.33. Readable in both user and privileged modes. 1 = The memory mapped in respective frames can be addressed by master with matching Master-ID. 0 = The memory is locked for master with matching Master-ID. PCR responds with AERROR. Writable only in privileged mode 1 = Sets the corresponding bit. 0 = Clears the corresponding bit. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Ppcs0mstidW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PPCS1MSTID` reader - 31:16\\]
There are 16 bits for each frame in PPCS. These bits sets the permission for maximum of 16 masters to address the memory mapped in each of the frame. The scheme is similar to the one described for PCSm MSTID in section 1.7.33. Readable in both user and privileged modes. 1 = The memory mapped in respective frames can be addressed by master with matching Master-ID. 0 = The memory is locked for master with matching Master-ID. PCR responds with AERROR. Writable only in privileged mode 1 = Sets the corresponding bit. 0 = Clears the corresponding bit. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Ppcs1mstidR = crate::FieldReader<u16>;
#[doc = "Field `PPCS1MSTID` writer - 31:16\\]
There are 16 bits for each frame in PPCS. These bits sets the permission for maximum of 16 masters to address the memory mapped in each of the frame. The scheme is similar to the one described for PCSm MSTID in section 1.7.33. Readable in both user and privileged modes. 1 = The memory mapped in respective frames can be addressed by master with matching Master-ID. 0 = The memory is locked for master with matching Master-ID. PCR responds with AERROR. Writable only in privileged mode 1 = Sets the corresponding bit. 0 = Clears the corresponding bit. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Ppcs1mstidW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
There are 16 bits for each frame in PPCS. These bits sets the permission for maximum of 16 masters to address the memory mapped in each of the frame. The scheme is similar to the one described for PCSm MSTID in section 1.7.33. Readable in both user and privileged modes. 1 = The memory mapped in respective frames can be addressed by master with matching Master-ID. 0 = The memory is locked for master with matching Master-ID. PCR responds with AERROR. Writable only in privileged mode 1 = Sets the corresponding bit. 0 = Clears the corresponding bit. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    pub fn ppcs0mstid(&self) -> Ppcs0mstidR {
        Ppcs0mstidR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
There are 16 bits for each frame in PPCS. These bits sets the permission for maximum of 16 masters to address the memory mapped in each of the frame. The scheme is similar to the one described for PCSm MSTID in section 1.7.33. Readable in both user and privileged modes. 1 = The memory mapped in respective frames can be addressed by master with matching Master-ID. 0 = The memory is locked for master with matching Master-ID. PCR responds with AERROR. Writable only in privileged mode 1 = Sets the corresponding bit. 0 = Clears the corresponding bit. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    pub fn ppcs1mstid(&self) -> Ppcs1mstidR {
        Ppcs1mstidR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
There are 16 bits for each frame in PPCS. These bits sets the permission for maximum of 16 masters to address the memory mapped in each of the frame. The scheme is similar to the one described for PCSm MSTID in section 1.7.33. Readable in both user and privileged modes. 1 = The memory mapped in respective frames can be addressed by master with matching Master-ID. 0 = The memory is locked for master with matching Master-ID. PCR responds with AERROR. Writable only in privileged mode 1 = Sets the corresponding bit. 0 = Clears the corresponding bit. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    #[must_use]
    pub fn ppcs0mstid(&mut self) -> Ppcs0mstidW<Ppcs0mstidSpec> {
        Ppcs0mstidW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
There are 16 bits for each frame in PPCS. These bits sets the permission for maximum of 16 masters to address the memory mapped in each of the frame. The scheme is similar to the one described for PCSm MSTID in section 1.7.33. Readable in both user and privileged modes. 1 = The memory mapped in respective frames can be addressed by master with matching Master-ID. 0 = The memory is locked for master with matching Master-ID. PCR responds with AERROR. Writable only in privileged mode 1 = Sets the corresponding bit. 0 = Clears the corresponding bit. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    #[must_use]
    pub fn ppcs1mstid(&mut self) -> Ppcs1mstidW<Ppcs0mstidSpec> {
        Ppcs1mstidW::new(self, 16)
    }
}
#[doc = "Memory Frame Master ID Protection Register32\n\nYou can [`read`](crate::Reg::read) this register and get [`ppcs0mstid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppcs0mstid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ppcs0mstidSpec;
impl crate::RegisterSpec for Ppcs0mstidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppcs0mstid::R`](R) reader structure"]
impl crate::Readable for Ppcs0mstidSpec {}
#[doc = "`write(|w| ..)` method takes [`ppcs0mstid::W`](W) writer structure"]
impl crate::Writable for Ppcs0mstidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PPCS0MSTID to value 0"]
impl crate::Resettable for Ppcs0mstidSpec {
    const RESET_VALUE: u32 = 0;
}
