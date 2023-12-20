// Copyright 2023 Marsh J. Ray
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![allow(dead_code)] //? TODO for development
#![allow(unused_mut)] //? TODO for development
#![allow(unused_variables)] //? TODO for development
#![allow(unused_imports)] //? TODO for development
#![allow(non_snake_case)] //? TODO for development
#![allow(clippy::new_without_default)] //? TODO for development
#![allow(clippy::let_and_return)] //? TODO for development
#![allow(clippy::redundant_closure)] //? TODO for development
#![allow(clippy::too_many_arguments)]

//? use std::any::Any;
//? use std::borrow::Cow;
//? use std::fmt::{Debug, Display};
//? use std::ops::RangeInclusive;
//? use std::sync::Arc;
//? use std::time::Instant;

//? use anyhow::{anyhow, bail, ensure, Context, Result};
//? use derive_more::Display;
//? use log::{debug, error, info, trace, warn};
//? use num_integer::Integer;
//? use num_rational::Ratio;
//? use num_traits::identities::Zero;
//? use once_cell::sync::Lazy;
//? use serde::{Deserialize, Serialize};
//? use strum::{self, EnumProperty, EnumString};

//========================================================================= Misc

trait IdentifiedObject { }

trait AbstractObject { }

/// "“any GML object having identity”"
trait AbstractGML: AbstractObject {
    /// group StandardObjectProperties
    /// 
    /// opt description: StringOrRefType "a text description of the object"
    ///
    ///  opt descriptionReference "a remote text description of the object"
    /// 
    ///
    /// opt identifier: CodeWithAuthorityType "Often, a special identifier is assigned to an object by the authority that maintains the feature with the intention that it is used in references to the object"
    /// 
    /// 0..N name "a label or identifier for the object, commonly a descriptive name"
    /// 
    /// id: XML ID
}

struct AbstractValue { }
struct AbstractCoverage { }
struct AbstractTopology { }
struct AbstractCRS { }

struct AssociationAttributeGroup {
    /// attributeGroup xlink:simpleLink
    ///     type = href, role, arcrole, title, show, actuate
    /// 
    /// nilReason: NilReason "may be used in a property element that is nillable to indicate a reason for a nil value"
    /// 
    /// remoteSchema

}

/// "An instance of this type shall contain an element representing an object, or serve as a pointer to a remote object"
/// AssociationRoleType

enum NilReason {
    Inapplicable,
    Missing,
    Template,
    Unknown,
    Withheld,
    Other(String),
}

enum Sign { Minus, Plus }

struct Code {
    /// codeSpace: AnyURI
}

/// "requires that the codeSpace attribute is provided"
struct CodeWithAuthority {
    // codeSpace: AnyURI
}

struct MeasureType {
    // uom (required): UomIdentifier
}


struct AbstractFeature { }

//========================================================================= 0D 1D Geometry


//========================================================================= 2D Geometry

//========================================================================= 3D Shape

struct Ellipsoid {
    // semiMajorAxis: MeasureType suitable for length
    
    // secondDefiningParameter one of:
    //     inverseFlattening: MeasureType
    //     semiMinorAxis: MeasureType suitable for length
    //     isSphere
}

//========================================================================= CS

trait CoordinateSystem {
}

struct EllipsoidalCS; impl CoordinateSystem for EllipsoidalCS { }
struct VerticalCS; impl CoordinateSystem for VerticalCS { }
struct CartesianCS; impl CoordinateSystem for CartesianCS { }
struct AffineCS; impl CoordinateSystem for AffineCS { }
struct LinearCS; impl CoordinateSystem for LinearCS { }
struct PolarCS; impl CoordinateSystem for PolarCS { }
struct SphericalCS; impl CoordinateSystem for SphericalCS { }
struct CylindricalCS; impl CoordinateSystem for CylindricalCS { }

/// ISO 19108 TM_CoordinateSystem
struct TimeCS {
    // one of:
    //     origin: DateTime (instant)
    //     originPosition: DateTime (instant)
    // 
    // interval: string, TimeIntervalLength
}
impl CoordinateSystem for TimeCS { }

/// TM_OrdinalReferenceSystem
struct TimeOrdinalReferenceSystem {
    // 1..N component: TimeOrdinalEra
}

struct TimeOrdinalEra {
    // From TimeEdge:
    //    id (mandatory)
    //    opt description
    //
    // name: string
    // 
    // opt begin: DateTime
    // 
    // opt end: DateTime
}

struct UserDefinedCS; impl CoordinateSystem for UserDefinedCS { }

//========================================================================= Datum

struct PrimeMeridian {
    /// greenwichLongitude: AngleType
    /// "If the value of the prime meridian "name" is "Greenwich" then the value shall be 0 degrees"
}

trait Datum {
    // opt domainOfValidity
    // 0..N scope

    // opt anchorDefinition
    // description of the origin (on earth)
    // codeSpace attribute may ref more detailed on the point or surface, or a set

    // opt realizationEpoch
    /// "realizationEpoch is the time after which this datum definition is valid.
    /// See ISO 19111:2007, Table 33, for details"
}

/// 
struct GeodeticDatum {
    /// The anchor is also known as the "fundamental point"
    /// "traditionally the point where the relationship between geoid and ellipsoid is defined.
    /// In some cases, the "fundamental point" may consist of a number of points. In those cases,
    /// the parameters defining the geoid/ellipsoid relationship have been averaged for these
    /// points, and the averages adopted as the datum definition"

    /// ref to primeMeridian

    /// ref to ellipsoid
}
impl Datum for GeodeticDatum { }

struct VerticalDatum {
    /// "a textual description and/or a set of parameters identifying a particular reference level
    /// surface used as a zero-height surface, including its position with respect to the Earth
    /// for any of the height types recognized by this International Standard"
}
impl Datum for VerticalDatum { }

/// "defines the origin of a Temporal Reference System"
/// "the anchor is not defined. Instead, a temporal datum carries a separate time origin of type DateTime"
/// "omits the "anchorDefinition" and "realizationEpoch" elements and adds the "origin" element with the dateTime type."
/// 
struct TemporalDatum {
    /// id required
    /// 
    /// 0..N metaDataProperty [deprecated]
    /// 
    /// opt description [partially deprecated]
    /// 
    /// opt descriptionReference
    /// 
    /// identifier
    /// 
    /// opt name
    /// 
    /// opt remarks
    /// 
    /// opt domainOfValidity
    /// 
    /// 0..N scope
    /// 
    /// 
    /// origin: DateTime
}
impl Datum for TemporalDatum { }

struct EngineeringDatum {
    /// "the anchor definition may be a physical point, or it may be a point with defined coordinates in another CRS"
}
impl Datum for EngineeringDatum { }

struct ImageDatum {
    /// " the anchor definition is usually either the centre of the image or the corner of the image."
    /// 
    /// pixelInCell
    /// "a specification of the way an image grid is associated with the image data attributes. The
    /// required codeSpace attribute shall reference a source of information specifying the values
    /// and meanings of all the allowed string values for this property"
}
impl Datum for ImageDatum { }

//========================================================================= CRS
struct AbstractCRS;

trait SingleCRS {
    //fn datum(&self) -> &impl Datum;
}

trait TraitDerivedCRS {

}

/// "describing the position of points through two or more independent coordinate reference systems"
struct CompoundCRS {
    // references two or more CRS
}
impl SingleCRS for CompoundCRS { }

/// A CRS "based on a geodetic datum"
struct GeodeticCRS {
    // ref to one of either an EllipsoidalCS, CartesianCS, or SphericalCS
    // ref to a geodeticDatum
}
impl SingleCRS for GeodeticCRS { }

/// "a 2D coordinate reference system used to approximate the shape of the Earth on a planar
/// surface, but in such a way that the distortion that is inherent to the approximation is
/// carefully controlled and known. Distortion correction is commonly applied to calculated
/// bearings and distances to produce values that are a close match to actual field values."
struct ProjectedCRS {
    // ref to one of either a base GeodeticCRS, or a base GeographicCRS
    // ref to a CartesianCS
}
impl TraitDerivedCRS for ProjectedCRS { }
impl SingleCRS for ProjectedCRS { }


/// "a contextually local coordinate reference system which can be divided into two broad categories:
/// (1) Earth-fixed systems applied to engineering activities on or near the surface of the Earth;
/// (2) CRSs on moving platforms such as road vehicles, vessels, aircraft, or spacecraft, see ISO 19111:2007, 8.3."
struct EngineeringCRS {
    // ref to one of AffineCS, CartesianCS, CylindricalCS, LinearCS, PolarCS, SphericalCS, or UserDefinedCS.
    // ref to a EngineeringDatum
}
impl SingleCRS for EngineeringCRS { }


/// "applied to locations in images. Image coordinate reference systems are treated as a separate
/// subtype because the definition of the associated image datum contains two attributes not
/// relevant to other engineering datums"
struct ImageCRS  {
    // ref to one of AffineCS or CartesianCS
    // ref to a ImageDatum
}
impl SingleCRS for ImageCRS { }

/// "a 1D coordinate reference system used for recording heights or depths. Vertical CRSs make use
/// of the direction of gravity to define the concept of height or depth, but the relationship with
/// gravity may not be straightforward. By implication, ellipsoidal heights (h) cannot be captured
/// in a vertical coordinate reference system. Ellipsoidal heights cannot exist independently, but
/// only as an inseparable part of a 3D coordinate tuple defined in a geographic 3D coordinate
/// reference system."
struct VerticalCRS{
    // ref to a VerticalCS
    // ref to a VerticalDatum
}
impl SingleCRS for VerticalCRS { }

/// A "1D coordinate reference system used for the recording of time".
struct TemporalCRS {
    // ref to a TimeCS, or "usesTemporalCS"?
    // ref to a TemporalDatum
}
impl SingleCRS for TemporalCRS { }

/// "defined by its coordinate conversion from another single coordinate reference system known
/// as the base CRS. The base CRS can be a projected coordinate reference system, if this
/// DerivedCRS is used for a georectified grid coverage as described in ISO 19123:2005,
/// Clause 8"
struct DerivedCRS {
    // conversion GeneralConversionPropertyType
    // ref to a base CRS. "SingleCRSPropertyType" Typically should be a SingleCRS, but can be a ProjectedCRS if this DerivedCRS is used for a "georectified grid coverage"
    // the type of a derived CRS "code space" "CodeWithAuthorityType"
    // `uses_cs` ref to the CoordinateSystem used by this CRS
}
impl TraitDerivedCRS for DerivedCRS { }
impl SingleCRS for DerivedCRS { }

//========================================================================= Operations

/// "a mathematical operation on coordinates that transforms or converts coordinates to another
/// coordinate reference system"
trait CoordinateOperation: IdentifiedObject {
    /// opt domainOfValidity
    /// 
    /// 1..N scope maxOccurs="unbounded" 
    /// 
    /// opt operationVersion
    /// 
    /// 0..N coordinateOperationAccuracy
    ///     "DQ_PositionalAccuracy object as encoded in ISO/TS 19139, either referencing or containing
    ///     the definition of that positional accuracy. That object contains an estimate of the impact
    ///     of this coordinate operation on point positional accuracy. That is, it gives position error
    ///     estimates for the target coordinates of this coordinate operation, assuming no errors in
    ///     the source coordinates"
    /// 
    /// opt ref to sourceCRS 
    /// 
    /// opt ref to targetCRS
    /// 
}

trait AbstractSingleOperation { }

/// "operation on coordinates that does not include any change of datum." E.g., map projection.
/// 
trait AbstractGeneralConversion {
    /// id (required)
    /// 
    /// usesMethod (required) ref to OperationMethod
    /// "All concrete types derived from this type shall extend this type to include a "usesMethod"
    /// element that references the "OperationMethod" element"
    /// 
    /// 0..N uses T: AbstractGeneralParameterValue
    /// "all concrete types derived from this type shall extend this type to include zero or more
    /// elements each named "uses...Value" that each use the type of an element substitutable for
    /// the AbstractGeneralParameterValue element"
    /// 
    /// 0..N metaDataProperty
    /// 
    /// opt description
    /// 
    /// opt descriptionReference
    /// 
    /// identifier
    /// 
    /// 0..N name
    /// 
    /// opt remarks
    /// 
    /// opt domainOfValidity
    /// 
    /// 1..N scope
    /// 
    /// 0..N coordinateOperationAccuracy
    /// 
}

/// "a property type for association roles to a general conversion, either referencing or containing the definition of that conversion"
struct GeneralConversionProperty {
    /// 0..N AbstractGeneralConversion
    /// 
    /// ref to AssociationAttributeGroup
}

/// "an abstract operation on coordinates that usually includes a change of Datum. The parameters
/// of a coordinate transformation are empirically derived from data containing the coordinates
/// of a series of points in both coordinate reference systems. This computational process is
/// usually "over-determined", allowing derivation of error (or accuracy) estimates for the
/// transformation. Also, the stochastic nature of the parameters may result in multiple
/// (different) versions of the same coordinate transformation. The gml:operationVersion,
/// gml:sourceCRS, and gml:targetCRS property elements are mandatory in a coordinate
/// transformation.
/// This abstract complex type is expected to be extended for well-known
/// operation methods with many Transformation instances, in Application Schemas that define
/// operation-method-specialized value element names and contents. This transformation uses
/// an operation method with associated parameter values. However, operation methods and
/// parameter values are directly associated with concrete subtypes, not with this abstract
/// type. All concrete types derived from this type shall extend this type to include a
/// "usesMethod" element that references one "OperationMethod" element. Similarly, all concrete
/// types derived from this type shall extend this type to include one or more elements each
/// named "uses...Value" that each use the type of an element substitutable for the
/// "AbstractGeneralParameterValue" element."
trait AbstractGeneralTransformation {

}

struct GeneralTransformationProperty {
    /// 0..N AbstractGeneralConversion
    /// 
    /// ref to AssociationAttributeGroup
}



/// "an ordered sequence of two or more coordinate operations. This sequence of operations is 
/// constrained by the requirement that the source coordinate reference system of step (n+1) 
/// must be the same as the target coordinate reference system of step (n). The source coordinate 
/// reference system of the first step and the target coordinate reference system of the last step 
/// are the source and target coordinate reference system associated with the concatenated 
/// operation. Instead of a forward operation, an inverse operation may be used for one or more 
/// of the operation steps mentioned above, if the inverse operation is uniquely defined by the 
/// forward operation."
/// 
struct ConcatenatedOperation {
    /// AbstractCoordinateOperationType stuff
    /// 
    /// 2..N CoordinateOperationPropertyType "an ordered sequence of 
    /// associations to the two or more operations used by this concatenated operation.
    /// 
    /// AggregationAttributeGroup "should be used to specify that the coordOperation 
    /// associations are ordered"
}
impl CoordinateOperation for ConcatenatedOperation { }

/// "specifies that a subset of a coordinate tuple is subject to a specific coordinate operation"
struct PassThroughOperation {
    /// AbstractCoordinateOperationType stuff
    /// 
    /// 1..N modifiedCoordinate: Nonnegative integer
    ///     "an ordered sequence of positive integers defining the positions in a coordinate tuple
    ///     of the coordinates affected by this pass-through operation"
    /// 
    /// coordOperation: CoordinateOperationPropertyType
    /// 
    /// AggregationAttributeGroup "should be used to specify that the modifiedCoordinate
    /// associations are ordered"

}
impl AbstractSingleOperation for PassThroughOperation { }
impl CoordinateOperation for PassThroughOperation { }

/// "a concrete object element derived from gml:AbstractGeneralTransformation (12.6.2.11).
/// This concrete object can be used for all operation methods, without using a GML
/// application schema that defines  operation-method-specialized element names and
/// contents, especially for methods with only one Transformation instance.
/// The gml:parameterValue elements are an unordered list of composition associations to
/// the set of parameter values used by this conversion operation"
struct Transformation {
    /// AbstractGeneralTransformationType
    /// 
    /// method
    /// 
    /// 0..N parameterValue
    /// "an unordered list of composition associations to the set of parameter values used by
    /// this conversion operation"
}
impl AbstractGeneralTransformation for Transformation { }
impl CoordinateOperation for Transformation { }

struct Conversion {
    /// AbstractGeneralConversionType stuff
    /// 
    /// ref to method: OperationMethodPropertyType
    /// 
    /// 0..N parameterValue: AbstractGeneralParameterValuePropertyType
}
impl AbstractGeneralConversion for Conversion { }
impl CoordinateOperation for Conversion { }

/// "The concrete XML elements that are substitutable for the CoordinateOperation element use
/// multiple lower-level elements containing data structures, including the elements named:"
struct CoordinateOperation{
    // OperationMethod
    // OperationParameter
    // OperationParameterGroup
    // ParameterValue
    // ParameterValueGroup
}

//========================================================================= Parameter values


/// "an abstract parameter value or group of parameter values"
/// 
/// "This abstract complexType is expected to be extended and restricted for well-known operation 
/// methods with many instances, in Application Schemas that define operation-method-specialized 
/// element names and contents."
trait AbstractGeneralParameterValue: AbstractObject {
    /// "Specific parameter value elements are directly contained in 
    /// concrete subtypes, not in this abstract type."
    /// "All concrete types derived from this type shall 
    /// extend this type to include one "...Value" element with an appropriate type, which should be 
    /// one of the element types allowed in the ParameterValueType. In addition, all derived concrete
    ///  types shall extend this type to include a "operationParameter" property element that
    ///  references one element substitutable for the "OperationParameter" object element."
}

/// "a parameter value, an ordered sequence of values, or a reference to a file of parameter values.
/// This concrete complex type may be used for operation methods without using an Application Schema
/// that defines operation-method-specialized element names and contents, especially for methods
/// with only one instance. This complex type may be used, extended, or restricted for well-known
/// operation methods, especially for methods with many instances"
struct ParameterValue {
    /// AbstractGeneralParameterValueType stuff
    /// 
    /// one of:
    ///     value:MeasureType
    ///     stringValue: string
    ///     integerValue: positive integer
    ///     booleanValue: bool
    ///     valueList: MeasureListType
    ///     integerValueList: integerList
    ///     valueFile: anyURI
    ///     dmsAngleValue [deprecated]
    /// 
    /// operationParameter: OperationParameterPropertyType
    ///     ref to "the operation parameter of which this is a value"
}
impl AbstractGeneralParameterValue for ParameterValue {}






