#[doc = "Register `PCS22MSTID` reader"]
pub type R = crate::R<Pcs22mstidSpec>;
#[doc = "Register `PCS22MSTID` writer"]
pub type W = crate::W<Pcs22mstidSpec>;
#[doc = "Field `PCS44MSTID` reader - 15:0\\]
There are 16 bits for each frame in PCS. These bits sets the permission for maximum of 16 masters to address the memory mapped in each of the frame. The following examples shows the usage of these register bits. (a) If bits 15:0 is 1010_1010_1010_1010, memory frame mapped to PCSm can be addressed by Masters with Master-ID equals 1,3,5,7,9,11,13,15. (b) If bits 31:24 is 1100_1100_1100_1100, memory frame mapped to PCS(m+1) can be addressed by Masters with Master-ID equals 2,3,6,7,10,11,14,15. Readable in both user and privileged modes. 1 = The memory mapped in respective frames can be addressed by master with matching Master-ID. 0 = The memory is locked for master with matching Master-ID. PCR responds with AERRORr. Writable only in privileged mode 1 = Sets the corresponding bit. 0 = Clears the corresponding bit. Writes to unimplemented bits have no effect and reads yield 0"]
pub type Pcs44mstidR = crate::FieldReader<u16>;
#[doc = "Field `PCS44MSTID` writer - 15:0\\]
There are 16 bits for each frame in PCS. These bits sets the permission for maximum of 16 masters to address the memory mapped in each of the frame. The following examples shows the usage of these register bits. (a) If bits 15:0 is 1010_1010_1010_1010, memory frame mapped to PCSm can be addressed by Masters with Master-ID equals 1,3,5,7,9,11,13,15. (b) If bits 31:24 is 1100_1100_1100_1100, memory frame mapped to PCS(m+1) can be addressed by Masters with Master-ID equals 2,3,6,7,10,11,14,15. Readable in both user and privileged modes. 1 = The memory mapped in respective frames can be addressed by master with matching Master-ID. 0 = The memory is locked for master with matching Master-ID. PCR responds with AERRORr. Writable only in privileged mode 1 = Sets the corresponding bit. 0 = Clears the corresponding bit. Writes to unimplemented bits have no effect and reads yield 0"]
pub type Pcs44mstidW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PCS45MSTID` reader - 31:16\\]
There are 16 bits for each frame in PCS. These bits sets the permission for maximum of 16 masters to address the memory mapped in each of the frame. The following examples shows the usage of these register bits. (a) If bits 15:0 is 1010_1010_1010_1010, memory frame mapped to PCSm can be addressed by Masters with Master-ID equals 1,3,5,7,9,11,13,15. (b) If bits 31:24 is 1100_1100_1100_1100, memory frame mapped to PCS(m+1) can be addressed by Masters with Master-ID equals 2,3,6,7,10,11,14,15. Readable in both user and privileged modes. 1 = The memory mapped in respective frames can be addressed by master with matching Master-ID. 0 = The memory is locked for master with matching Master-ID. PCR responds with AERRORr. Writable only in privileged mode 1 = Sets the corresponding bit. 0 = Clears the corresponding bit. Writes to unimplemented bits have no effect and reads yield 0"]
pub type Pcs45mstidR = crate::FieldReader<u16>;
#[doc = "Field `PCS45MSTID` writer - 31:16\\]
There are 16 bits for each frame in PCS. These bits sets the permission for maximum of 16 masters to address the memory mapped in each of the frame. The following examples shows the usage of these register bits. (a) If bits 15:0 is 1010_1010_1010_1010, memory frame mapped to PCSm can be addressed by Masters with Master-ID equals 1,3,5,7,9,11,13,15. (b) If bits 31:24 is 1100_1100_1100_1100, memory frame mapped to PCS(m+1) can be addressed by Masters with Master-ID equals 2,3,6,7,10,11,14,15. Readable in both user and privileged modes. 1 = The memory mapped in respective frames can be addressed by master with matching Master-ID. 0 = The memory is locked for master with matching Master-ID. PCR responds with AERRORr. Writable only in privileged mode 1 = Sets the corresponding bit. 0 = Clears the corresponding bit. Writes to unimplemented bits have no effect and reads yield 0"]
pub type Pcs45mstidW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
There are 16 bits for each frame in PCS. These bits sets the permission for maximum of 16 masters to address the memory mapped in each of the frame. The following examples shows the usage of these register bits. (a) If bits 15:0 is 1010_1010_1010_1010, memory frame mapped to PCSm can be addressed by Masters with Master-ID equals 1,3,5,7,9,11,13,15. (b) If bits 31:24 is 1100_1100_1100_1100, memory frame mapped to PCS(m+1) can be addressed by Masters with Master-ID equals 2,3,6,7,10,11,14,15. Readable in both user and privileged modes. 1 = The memory mapped in respective frames can be addressed by master with matching Master-ID. 0 = The memory is locked for master with matching Master-ID. PCR responds with AERRORr. Writable only in privileged mode 1 = Sets the corresponding bit. 0 = Clears the corresponding bit. Writes to unimplemented bits have no effect and reads yield 0"]
    #[inline(always)]
    pub fn pcs44mstid(&self) -> Pcs44mstidR {
        Pcs44mstidR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
There are 16 bits for each frame in PCS. These bits sets the permission for maximum of 16 masters to address the memory mapped in each of the frame. The following examples shows the usage of these register bits. (a) If bits 15:0 is 1010_1010_1010_1010, memory frame mapped to PCSm can be addressed by Masters with Master-ID equals 1,3,5,7,9,11,13,15. (b) If bits 31:24 is 1100_1100_1100_1100, memory frame mapped to PCS(m+1) can be addressed by Masters with Master-ID equals 2,3,6,7,10,11,14,15. Readable in both user and privileged modes. 1 = The memory mapped in respective frames can be addressed by master with matching Master-ID. 0 = The memory is locked for master with matching Master-ID. PCR responds with AERRORr. Writable only in privileged mode 1 = Sets the corresponding bit. 0 = Clears the corresponding bit. Writes to unimplemented bits have no effect and reads yield 0"]
    #[inline(always)]
    pub fn pcs45mstid(&self) -> Pcs45mstidR {
        Pcs45mstidR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
There are 16 bits for each frame in PCS. These bits sets the permission for maximum of 16 masters to address the memory mapped in each of the frame. The following examples shows the usage of these register bits. (a) If bits 15:0 is 1010_1010_1010_1010, memory frame mapped to PCSm can be addressed by Masters with Master-ID equals 1,3,5,7,9,11,13,15. (b) If bits 31:24 is 1100_1100_1100_1100, memory frame mapped to PCS(m+1) can be addressed by Masters with Master-ID equals 2,3,6,7,10,11,14,15. Readable in both user and privileged modes. 1 = The memory mapped in respective frames can be addressed by master with matching Master-ID. 0 = The memory is locked for master with matching Master-ID. PCR responds with AERRORr. Writable only in privileged mode 1 = Sets the corresponding bit. 0 = Clears the corresponding bit. Writes to unimplemented bits have no effect and reads yield 0"]
    #[inline(always)]
    #[must_use]
    pub fn pcs44mstid(&mut self) -> Pcs44mstidW<Pcs22mstidSpec> {
        Pcs44mstidW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
There are 16 bits for each frame in PCS. These bits sets the permission for maximum of 16 masters to address the memory mapped in each of the frame. The following examples shows the usage of these register bits. (a) If bits 15:0 is 1010_1010_1010_1010, memory frame mapped to PCSm can be addressed by Masters with Master-ID equals 1,3,5,7,9,11,13,15. (b) If bits 31:24 is 1100_1100_1100_1100, memory frame mapped to PCS(m+1) can be addressed by Masters with Master-ID equals 2,3,6,7,10,11,14,15. Readable in both user and privileged modes. 1 = The memory mapped in respective frames can be addressed by master with matching Master-ID. 0 = The memory is locked for master with matching Master-ID. PCR responds with AERRORr. Writable only in privileged mode 1 = Sets the corresponding bit. 0 = Clears the corresponding bit. Writes to unimplemented bits have no effect and reads yield 0"]
    #[inline(always)]
    #[must_use]
    pub fn pcs45mstid(&mut self) -> Pcs45mstidW<Pcs22mstidSpec> {
        Pcs45mstidW::new(self, 16)
    }
}
#[doc = "Memory Frame Master ID Protection Register22\n\nYou can [`read`](crate::Reg::read) this register and get [`pcs22mstid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcs22mstid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pcs22mstidSpec;
impl crate::RegisterSpec for Pcs22mstidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcs22mstid::R`](R) reader structure"]
impl crate::Readable for Pcs22mstidSpec {}
#[doc = "`write(|w| ..)` method takes [`pcs22mstid::W`](W) writer structure"]
impl crate::Writable for Pcs22mstidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCS22MSTID to value 0"]
impl crate::Resettable for Pcs22mstidSpec {
    const RESET_VALUE: u32 = 0;
}
