#[doc = "Register `PPS1MSTID_L` reader"]
pub type R = crate::R<Pps1mstidLSpec>;
#[doc = "Register `PPS1MSTID_L` writer"]
pub type W = crate::W<Pps1mstidLSpec>;
#[doc = "Field `PPS1_QUAD0_MSTID` reader - 15:0\\]
There are 16 bits for each quadrant in PPS frame. These bits set the permission for maximum of 16 masters to address the peripheral mapped in each of the quadrant. The scheme is similar to the one described for PS MSTID register in section 1.7.30. Readable in both user and privileged modes. 1 = The peripheral mapped in the quadrant can be addressed by master with matching Master-ID. 0 = The peripheral is locked for master with matching Master-ID. PCR responds with AERROR. Writable only in privileged mode 1 = Sets the corresponding bit. 0 = Clears the corresponding bit. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pps1Quad0MstidR = crate::FieldReader<u16>;
#[doc = "Field `PPS1_QUAD0_MSTID` writer - 15:0\\]
There are 16 bits for each quadrant in PPS frame. These bits set the permission for maximum of 16 masters to address the peripheral mapped in each of the quadrant. The scheme is similar to the one described for PS MSTID register in section 1.7.30. Readable in both user and privileged modes. 1 = The peripheral mapped in the quadrant can be addressed by master with matching Master-ID. 0 = The peripheral is locked for master with matching Master-ID. PCR responds with AERROR. Writable only in privileged mode 1 = Sets the corresponding bit. 0 = Clears the corresponding bit. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pps1Quad0MstidW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PPS1_QUAD1_MSTID` reader - 31:16\\]
There are 16 bits for each quadrant in PPS frame. These bits set the permission for maximum of 16 masters to address the peripheral mapped in each of the quadrant. The scheme is similar to the one described for PS MSTID register in section 1.7.30. Readable in both user and privileged modes. 1 = The peripheral mapped in the quadrant can be addressed by master with matching Master-ID. 0 = The peripheral is locked for master with matching Master-ID. PCR responds with AERROR. Writable only in privileged mode 1 = Sets the corresponding bit. 0 = Clears the corresponding bit. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pps1Quad1MstidR = crate::FieldReader<u16>;
#[doc = "Field `PPS1_QUAD1_MSTID` writer - 31:16\\]
There are 16 bits for each quadrant in PPS frame. These bits set the permission for maximum of 16 masters to address the peripheral mapped in each of the quadrant. The scheme is similar to the one described for PS MSTID register in section 1.7.30. Readable in both user and privileged modes. 1 = The peripheral mapped in the quadrant can be addressed by master with matching Master-ID. 0 = The peripheral is locked for master with matching Master-ID. PCR responds with AERROR. Writable only in privileged mode 1 = Sets the corresponding bit. 0 = Clears the corresponding bit. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pps1Quad1MstidW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
There are 16 bits for each quadrant in PPS frame. These bits set the permission for maximum of 16 masters to address the peripheral mapped in each of the quadrant. The scheme is similar to the one described for PS MSTID register in section 1.7.30. Readable in both user and privileged modes. 1 = The peripheral mapped in the quadrant can be addressed by master with matching Master-ID. 0 = The peripheral is locked for master with matching Master-ID. PCR responds with AERROR. Writable only in privileged mode 1 = Sets the corresponding bit. 0 = Clears the corresponding bit. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    pub fn pps1_quad0_mstid(&self) -> Pps1Quad0MstidR {
        Pps1Quad0MstidR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
There are 16 bits for each quadrant in PPS frame. These bits set the permission for maximum of 16 masters to address the peripheral mapped in each of the quadrant. The scheme is similar to the one described for PS MSTID register in section 1.7.30. Readable in both user and privileged modes. 1 = The peripheral mapped in the quadrant can be addressed by master with matching Master-ID. 0 = The peripheral is locked for master with matching Master-ID. PCR responds with AERROR. Writable only in privileged mode 1 = Sets the corresponding bit. 0 = Clears the corresponding bit. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    pub fn pps1_quad1_mstid(&self) -> Pps1Quad1MstidR {
        Pps1Quad1MstidR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
There are 16 bits for each quadrant in PPS frame. These bits set the permission for maximum of 16 masters to address the peripheral mapped in each of the quadrant. The scheme is similar to the one described for PS MSTID register in section 1.7.30. Readable in both user and privileged modes. 1 = The peripheral mapped in the quadrant can be addressed by master with matching Master-ID. 0 = The peripheral is locked for master with matching Master-ID. PCR responds with AERROR. Writable only in privileged mode 1 = Sets the corresponding bit. 0 = Clears the corresponding bit. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    #[must_use]
    pub fn pps1_quad0_mstid(&mut self) -> Pps1Quad0MstidW<Pps1mstidLSpec> {
        Pps1Quad0MstidW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
There are 16 bits for each quadrant in PPS frame. These bits set the permission for maximum of 16 masters to address the peripheral mapped in each of the quadrant. The scheme is similar to the one described for PS MSTID register in section 1.7.30. Readable in both user and privileged modes. 1 = The peripheral mapped in the quadrant can be addressed by master with matching Master-ID. 0 = The peripheral is locked for master with matching Master-ID. PCR responds with AERROR. Writable only in privileged mode 1 = Sets the corresponding bit. 0 = Clears the corresponding bit. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    #[must_use]
    pub fn pps1_quad1_mstid(&mut self) -> Pps1Quad1MstidW<Pps1mstidLSpec> {
        Pps1Quad1MstidW::new(self, 16)
    }
}
#[doc = "Privileged Peripheral Frame Master-ID Protection Register1_L\n\nYou can [`read`](crate::Reg::read) this register and get [`pps1mstid_l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pps1mstid_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pps1mstidLSpec;
impl crate::RegisterSpec for Pps1mstidLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pps1mstid_l::R`](R) reader structure"]
impl crate::Readable for Pps1mstidLSpec {}
#[doc = "`write(|w| ..)` method takes [`pps1mstid_l::W`](W) writer structure"]
impl crate::Writable for Pps1mstidLSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PPS1MSTID_L to value 0"]
impl crate::Resettable for Pps1mstidLSpec {
    const RESET_VALUE: u32 = 0;
}
