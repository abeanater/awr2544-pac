#[doc = "Register `PS16MSTID_L` reader"]
pub type R = crate::R<Ps16mstidLSpec>;
#[doc = "Register `PS16MSTID_L` writer"]
pub type W = crate::W<Ps16mstidLSpec>;
#[doc = "Field `PS16_QUAD0_MSTID` reader - 15:0\\]
There are 16 bits for each quadrant in PS frame. These bits sets the permission for maximum of 16 masters to address the peripheral mapped in each of the quadrant. The following examples shows the usage of these register bits. (a) If bits 15:0 is 1010_1010_1010_1010, peripheral mapped in Quad0 can be addressed by Masters with Master-ID equals 1,3,5,7,9,11,13,15 (b) If bits 31:16 is 1100_1100_1100_1100, peripheral mapped in Quad1 can be addressed by Masters with Master-ID equals 2,3,6,7,10,11,14,15 Readable in both user and privileged modes. 1 = The peripheral mapped in the quadrant can be addressed by masters with matching Master-ID. 0 = The peipheral is locked for masters with matching Master-ID. PCR responds with AERROR. Writable only in privileged mode 1 = Sets the corresponding bit. 0 = Clears the corresponding bit. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Ps16Quad0MstidR = crate::FieldReader<u16>;
#[doc = "Field `PS16_QUAD0_MSTID` writer - 15:0\\]
There are 16 bits for each quadrant in PS frame. These bits sets the permission for maximum of 16 masters to address the peripheral mapped in each of the quadrant. The following examples shows the usage of these register bits. (a) If bits 15:0 is 1010_1010_1010_1010, peripheral mapped in Quad0 can be addressed by Masters with Master-ID equals 1,3,5,7,9,11,13,15 (b) If bits 31:16 is 1100_1100_1100_1100, peripheral mapped in Quad1 can be addressed by Masters with Master-ID equals 2,3,6,7,10,11,14,15 Readable in both user and privileged modes. 1 = The peripheral mapped in the quadrant can be addressed by masters with matching Master-ID. 0 = The peipheral is locked for masters with matching Master-ID. PCR responds with AERROR. Writable only in privileged mode 1 = Sets the corresponding bit. 0 = Clears the corresponding bit. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Ps16Quad0MstidW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PS16_QUAD1_MSTID` reader - 31:16\\]
There are 16 bits for each quadrant in PS frame. These bits sets the permission for maximum of 16 masters to address the peripheral mapped in each of the quadrant. The following examples shows the usage of these register bits. (a) If bits 15:0 is 1010_1010_1010_1010, peripheral mapped in Quad0 can be addressed by Masters with Master-ID equals 1,3,5,7,9,11,13,15 (b) If bits 31:16 is 1100_1100_1100_1100, peripheral mapped in Quad1 can be addressed by Masters with Master-ID equals 2,3,6,7,10,11,14,15 Readable in both user and privileged modes. 1 = The peripheral mapped in the quadrant can be addressed by masters with matching Master-ID. 0 = The peipheral is locked for masters with matching Master-ID. PCR responds with AERROR. Writable only in privileged mode 1 = Sets the corresponding bit. 0 = Clears the corresponding bit. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Ps16Quad1MstidR = crate::FieldReader<u16>;
#[doc = "Field `PS16_QUAD1_MSTID` writer - 31:16\\]
There are 16 bits for each quadrant in PS frame. These bits sets the permission for maximum of 16 masters to address the peripheral mapped in each of the quadrant. The following examples shows the usage of these register bits. (a) If bits 15:0 is 1010_1010_1010_1010, peripheral mapped in Quad0 can be addressed by Masters with Master-ID equals 1,3,5,7,9,11,13,15 (b) If bits 31:16 is 1100_1100_1100_1100, peripheral mapped in Quad1 can be addressed by Masters with Master-ID equals 2,3,6,7,10,11,14,15 Readable in both user and privileged modes. 1 = The peripheral mapped in the quadrant can be addressed by masters with matching Master-ID. 0 = The peipheral is locked for masters with matching Master-ID. PCR responds with AERROR. Writable only in privileged mode 1 = Sets the corresponding bit. 0 = Clears the corresponding bit. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Ps16Quad1MstidW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
There are 16 bits for each quadrant in PS frame. These bits sets the permission for maximum of 16 masters to address the peripheral mapped in each of the quadrant. The following examples shows the usage of these register bits. (a) If bits 15:0 is 1010_1010_1010_1010, peripheral mapped in Quad0 can be addressed by Masters with Master-ID equals 1,3,5,7,9,11,13,15 (b) If bits 31:16 is 1100_1100_1100_1100, peripheral mapped in Quad1 can be addressed by Masters with Master-ID equals 2,3,6,7,10,11,14,15 Readable in both user and privileged modes. 1 = The peripheral mapped in the quadrant can be addressed by masters with matching Master-ID. 0 = The peipheral is locked for masters with matching Master-ID. PCR responds with AERROR. Writable only in privileged mode 1 = Sets the corresponding bit. 0 = Clears the corresponding bit. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    pub fn ps16_quad0_mstid(&self) -> Ps16Quad0MstidR {
        Ps16Quad0MstidR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
There are 16 bits for each quadrant in PS frame. These bits sets the permission for maximum of 16 masters to address the peripheral mapped in each of the quadrant. The following examples shows the usage of these register bits. (a) If bits 15:0 is 1010_1010_1010_1010, peripheral mapped in Quad0 can be addressed by Masters with Master-ID equals 1,3,5,7,9,11,13,15 (b) If bits 31:16 is 1100_1100_1100_1100, peripheral mapped in Quad1 can be addressed by Masters with Master-ID equals 2,3,6,7,10,11,14,15 Readable in both user and privileged modes. 1 = The peripheral mapped in the quadrant can be addressed by masters with matching Master-ID. 0 = The peipheral is locked for masters with matching Master-ID. PCR responds with AERROR. Writable only in privileged mode 1 = Sets the corresponding bit. 0 = Clears the corresponding bit. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    pub fn ps16_quad1_mstid(&self) -> Ps16Quad1MstidR {
        Ps16Quad1MstidR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
There are 16 bits for each quadrant in PS frame. These bits sets the permission for maximum of 16 masters to address the peripheral mapped in each of the quadrant. The following examples shows the usage of these register bits. (a) If bits 15:0 is 1010_1010_1010_1010, peripheral mapped in Quad0 can be addressed by Masters with Master-ID equals 1,3,5,7,9,11,13,15 (b) If bits 31:16 is 1100_1100_1100_1100, peripheral mapped in Quad1 can be addressed by Masters with Master-ID equals 2,3,6,7,10,11,14,15 Readable in both user and privileged modes. 1 = The peripheral mapped in the quadrant can be addressed by masters with matching Master-ID. 0 = The peipheral is locked for masters with matching Master-ID. PCR responds with AERROR. Writable only in privileged mode 1 = Sets the corresponding bit. 0 = Clears the corresponding bit. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    #[must_use]
    pub fn ps16_quad0_mstid(&mut self) -> Ps16Quad0MstidW<Ps16mstidLSpec> {
        Ps16Quad0MstidW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
There are 16 bits for each quadrant in PS frame. These bits sets the permission for maximum of 16 masters to address the peripheral mapped in each of the quadrant. The following examples shows the usage of these register bits. (a) If bits 15:0 is 1010_1010_1010_1010, peripheral mapped in Quad0 can be addressed by Masters with Master-ID equals 1,3,5,7,9,11,13,15 (b) If bits 31:16 is 1100_1100_1100_1100, peripheral mapped in Quad1 can be addressed by Masters with Master-ID equals 2,3,6,7,10,11,14,15 Readable in both user and privileged modes. 1 = The peripheral mapped in the quadrant can be addressed by masters with matching Master-ID. 0 = The peipheral is locked for masters with matching Master-ID. PCR responds with AERROR. Writable only in privileged mode 1 = Sets the corresponding bit. 0 = Clears the corresponding bit. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    #[must_use]
    pub fn ps16_quad1_mstid(&mut self) -> Ps16Quad1MstidW<Ps16mstidLSpec> {
        Ps16Quad1MstidW::new(self, 16)
    }
}
#[doc = "Peripheral Frame Master-ID Protection Register16_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ps16mstid_l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps16mstid_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ps16mstidLSpec;
impl crate::RegisterSpec for Ps16mstidLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ps16mstid_l::R`](R) reader structure"]
impl crate::Readable for Ps16mstidLSpec {}
#[doc = "`write(|w| ..)` method takes [`ps16mstid_l::W`](W) writer structure"]
impl crate::Writable for Ps16mstidLSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PS16MSTID_L to value 0"]
impl crate::Resettable for Ps16mstidLSpec {
    const RESET_VALUE: u32 = 0;
}
