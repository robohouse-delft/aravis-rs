// Generated by gir (https://github.com/gtk-rs/gir @ e8f82cf)
// from ../gir-files (@ 6f12897)
// DO NOT EDIT

#include "manual.h"
#include <stdio.h>

#define PRINT_CONSTANT(CONSTANT_NAME) \
    printf("%s;", #CONSTANT_NAME); \
    printf(_Generic((CONSTANT_NAME), \
                    char *: "%s", \
                    const char *: "%s", \
                    char: "%c", \
                    signed char: "%hhd", \
                    unsigned char: "%hhu", \
                    short int: "%hd", \
                    unsigned short int: "%hu", \
                    int: "%d", \
                    unsigned int: "%u", \
                    long: "%ld", \
                    unsigned long: "%lu", \
                    long long: "%lld", \
                    unsigned long long: "%llu", \
                    float: "%f", \
                    double: "%f", \
                    long double: "%ld"), \
           CONSTANT_NAME); \
    printf("\n");

int main() {
    PRINT_CONSTANT((gint) ARV_ACQUISITION_MODE_CONTINUOUS);
    PRINT_CONSTANT((gint) ARV_ACQUISITION_MODE_MULTI_FRAME);
    PRINT_CONSTANT((gint) ARV_ACQUISITION_MODE_SINGLE_FRAME);
    PRINT_CONSTANT((gint) ARV_AUTO_CONTINUOUS);
    PRINT_CONSTANT((gint) ARV_AUTO_OFF);
    PRINT_CONSTANT((gint) ARV_AUTO_ONCE);
    PRINT_CONSTANT((gint) ARV_BUFFER_PAYLOAD_TYPE_CHUNK_DATA);
    PRINT_CONSTANT((gint) ARV_BUFFER_PAYLOAD_TYPE_EXTENDED_CHUNK_DATA);
    PRINT_CONSTANT((gint) ARV_BUFFER_PAYLOAD_TYPE_FILE);
    PRINT_CONSTANT((gint) ARV_BUFFER_PAYLOAD_TYPE_H264);
    PRINT_CONSTANT((gint) ARV_BUFFER_PAYLOAD_TYPE_IMAGE);
    PRINT_CONSTANT((gint) ARV_BUFFER_PAYLOAD_TYPE_IMAGE_EXTENDED_CHUNK);
    PRINT_CONSTANT((gint) ARV_BUFFER_PAYLOAD_TYPE_JPEG);
    PRINT_CONSTANT((gint) ARV_BUFFER_PAYLOAD_TYPE_JPEG2000);
    PRINT_CONSTANT((gint) ARV_BUFFER_PAYLOAD_TYPE_MULTIZONE_IMAGE);
    PRINT_CONSTANT((gint) ARV_BUFFER_PAYLOAD_TYPE_RAWDATA);
    PRINT_CONSTANT((gint) ARV_BUFFER_PAYLOAD_TYPE_UNKNOWN);
    PRINT_CONSTANT((gint) ARV_BUFFER_STATUS_ABORTED);
    PRINT_CONSTANT((gint) ARV_BUFFER_STATUS_CLEARED);
    PRINT_CONSTANT((gint) ARV_BUFFER_STATUS_FILLING);
    PRINT_CONSTANT((gint) ARV_BUFFER_STATUS_MISSING_PACKETS);
    PRINT_CONSTANT((gint) ARV_BUFFER_STATUS_SIZE_MISMATCH);
    PRINT_CONSTANT((gint) ARV_BUFFER_STATUS_SUCCESS);
    PRINT_CONSTANT((gint) ARV_BUFFER_STATUS_TIMEOUT);
    PRINT_CONSTANT((gint) ARV_BUFFER_STATUS_UNKNOWN);
    PRINT_CONSTANT((gint) ARV_BUFFER_STATUS_WRONG_PACKET_ID);
    PRINT_CONSTANT((gint) ARV_CHUNK_PARSER_ERROR_BUFFER_NOT_FOUND);
    PRINT_CONSTANT((gint) ARV_CHUNK_PARSER_ERROR_CHUNK_NOT_FOUND);
    PRINT_CONSTANT((gint) ARV_CHUNK_PARSER_ERROR_INVALID_FEATURE_TYPE);
    PRINT_CONSTANT((gint) ARV_DEVICE_ERROR_FEATURE_NOT_FOUND);
    PRINT_CONSTANT((gint) ARV_DEVICE_ERROR_GENICAM_NOT_FOUND);
    PRINT_CONSTANT((gint) ARV_DEVICE_ERROR_INVALID_PARAMETER);
    PRINT_CONSTANT((gint) ARV_DEVICE_ERROR_NOT_CONNECTED);
    PRINT_CONSTANT((gint) ARV_DEVICE_ERROR_NOT_CONTROLLER);
    PRINT_CONSTANT((gint) ARV_DEVICE_ERROR_NOT_FOUND);
    PRINT_CONSTANT((gint) ARV_DEVICE_ERROR_NO_STREAM_CHANNEL);
    PRINT_CONSTANT((gint) ARV_DEVICE_ERROR_PROTOCOL_ERROR);
    PRINT_CONSTANT((gint) ARV_DEVICE_ERROR_TIMEOUT);
    PRINT_CONSTANT((gint) ARV_DEVICE_ERROR_TRANSFER_ERROR);
    PRINT_CONSTANT((gint) ARV_DEVICE_ERROR_UNKNOWN);
    PRINT_CONSTANT((gint) ARV_DEVICE_ERROR_WRONG_FEATURE);
    PRINT_CONSTANT((gint) ARV_DOM_NODE_TYPE_ATTRIBUTE_NODE);
    PRINT_CONSTANT((gint) ARV_DOM_NODE_TYPE_CDATA_SECTION_NODE);
    PRINT_CONSTANT((gint) ARV_DOM_NODE_TYPE_COMMENT_NODE);
    PRINT_CONSTANT((gint) ARV_DOM_NODE_TYPE_DOCUMENT_FRAGMENT_NODE);
    PRINT_CONSTANT((gint) ARV_DOM_NODE_TYPE_DOCUMENT_NODE);
    PRINT_CONSTANT((gint) ARV_DOM_NODE_TYPE_DOCUMENT_TYPE_NODE);
    PRINT_CONSTANT((gint) ARV_DOM_NODE_TYPE_ELEMENT_NODE);
    PRINT_CONSTANT((gint) ARV_DOM_NODE_TYPE_ENTITY_NODE);
    PRINT_CONSTANT((gint) ARV_DOM_NODE_TYPE_ENTITY_REFERENCE_NODE);
    PRINT_CONSTANT((gint) ARV_DOM_NODE_TYPE_NOTATION_NODE);
    PRINT_CONSTANT((gint) ARV_DOM_NODE_TYPE_PROCESSING_INSTRUCTION_NODE);
    PRINT_CONSTANT((gint) ARV_DOM_NODE_TYPE_TEXT_NODE);
    PRINT_CONSTANT((gint) ARV_EXPOSURE_MODE_OFF);
    PRINT_CONSTANT((gint) ARV_EXPOSURE_MODE_TIMED);
    PRINT_CONSTANT((gint) ARV_EXPOSURE_MODE_TRIGGER_CONTROLLED);
    PRINT_CONSTANT((gint) ARV_EXPOSURE_MODE_TRIGGER_WIDTH);
    PRINT_CONSTANT(ARV_FAKE_CAMERA_ACQUISITION_FRAME_RATE_DEFAULT);
    PRINT_CONSTANT(ARV_FAKE_CAMERA_BINNING_HORIZONTAL_DEFAULT);
    PRINT_CONSTANT(ARV_FAKE_CAMERA_BINNING_VERTICAL_DEFAULT);
    PRINT_CONSTANT(ARV_FAKE_CAMERA_EXPOSURE_TIME_US_DEFAULT);
    PRINT_CONSTANT(ARV_FAKE_CAMERA_HEIGHT_DEFAULT);
    PRINT_CONSTANT(ARV_FAKE_CAMERA_MEMORY_SIZE);
    PRINT_CONSTANT(ARV_FAKE_CAMERA_REGISTER_ACQUISITION);
    PRINT_CONSTANT(ARV_FAKE_CAMERA_REGISTER_ACQUISITION_FRAME_PERIOD_US);
    PRINT_CONSTANT(ARV_FAKE_CAMERA_REGISTER_ACQUISITION_MODE);
    PRINT_CONSTANT(ARV_FAKE_CAMERA_REGISTER_ACQUISITION_START_OFFSET);
    PRINT_CONSTANT(ARV_FAKE_CAMERA_REGISTER_BINNING_HORIZONTAL);
    PRINT_CONSTANT(ARV_FAKE_CAMERA_REGISTER_BINNING_VERTICAL);
    PRINT_CONSTANT(ARV_FAKE_CAMERA_REGISTER_EXPOSURE_TIME_US);
    PRINT_CONSTANT(ARV_FAKE_CAMERA_REGISTER_FRAME_START_OFFSET);
    PRINT_CONSTANT(ARV_FAKE_CAMERA_REGISTER_GAIN_MODE);
    PRINT_CONSTANT(ARV_FAKE_CAMERA_REGISTER_GAIN_RAW);
    PRINT_CONSTANT(ARV_FAKE_CAMERA_REGISTER_HEIGHT);
    PRINT_CONSTANT(ARV_FAKE_CAMERA_REGISTER_PIXEL_FORMAT);
    PRINT_CONSTANT(ARV_FAKE_CAMERA_REGISTER_SENSOR_HEIGHT);
    PRINT_CONSTANT(ARV_FAKE_CAMERA_REGISTER_SENSOR_WIDTH);
    PRINT_CONSTANT(ARV_FAKE_CAMERA_REGISTER_TEST);
    PRINT_CONSTANT(ARV_FAKE_CAMERA_REGISTER_TRIGGER_ACTIVATION);
    PRINT_CONSTANT(ARV_FAKE_CAMERA_REGISTER_TRIGGER_MODE);
    PRINT_CONSTANT(ARV_FAKE_CAMERA_REGISTER_TRIGGER_SOURCE);
    PRINT_CONSTANT(ARV_FAKE_CAMERA_REGISTER_WIDTH);
    PRINT_CONSTANT(ARV_FAKE_CAMERA_REGISTER_X_OFFSET);
    PRINT_CONSTANT(ARV_FAKE_CAMERA_REGISTER_Y_OFFSET);
    PRINT_CONSTANT(ARV_FAKE_CAMERA_SENSOR_HEIGHT);
    PRINT_CONSTANT(ARV_FAKE_CAMERA_SENSOR_WIDTH);
    PRINT_CONSTANT(ARV_FAKE_CAMERA_TEST_REGISTER_DEFAULT);
    PRINT_CONSTANT(ARV_FAKE_CAMERA_WIDTH_DEFAULT);
    PRINT_CONSTANT((gint) ARV_GC_ACCESS_MODE_RO);
    PRINT_CONSTANT((gint) ARV_GC_ACCESS_MODE_RW);
    PRINT_CONSTANT((gint) ARV_GC_ACCESS_MODE_UNDEFINED);
    PRINT_CONSTANT((gint) ARV_GC_ACCESS_MODE_WO);
    PRINT_CONSTANT((gint) ARV_GC_CACHABLE_NO_CACHE);
    PRINT_CONSTANT((gint) ARV_GC_CACHABLE_UNDEFINED);
    PRINT_CONSTANT((gint) ARV_GC_CACHABLE_WRITE_AROUND);
    PRINT_CONSTANT((gint) ARV_GC_CACHABLE_WRITE_THROUGH);
    PRINT_CONSTANT((gint) ARV_GC_DISPLAY_NOTATION_AUTOMATIC);
    PRINT_CONSTANT((gint) ARV_GC_DISPLAY_NOTATION_FIXED);
    PRINT_CONSTANT((gint) ARV_GC_DISPLAY_NOTATION_SCIENTIFIC);
    PRINT_CONSTANT((gint) ARV_GC_DISPLAY_NOTATION_UNDEFINED);
    PRINT_CONSTANT((gint) ARV_GC_ERROR_EMPTY_ENUMERATION);
    PRINT_CONSTANT((gint) ARV_GC_ERROR_ENUM_ENTRY_NOT_FOUND);
    PRINT_CONSTANT((gint) ARV_GC_ERROR_GET_AS_STRING_UNDEFINED);
    PRINT_CONSTANT((gint) ARV_GC_ERROR_INVALID_BIT_RANGE);
    PRINT_CONSTANT((gint) ARV_GC_ERROR_INVALID_LENGTH);
    PRINT_CONSTANT((gint) ARV_GC_ERROR_INVALID_PVALUE);
    PRINT_CONSTANT((gint) ARV_GC_ERROR_NODE_NOT_FOUND);
    PRINT_CONSTANT((gint) ARV_GC_ERROR_NO_DEVICE_SET);
    PRINT_CONSTANT((gint) ARV_GC_ERROR_NO_EVENT_IMPLEMENTATION);
    PRINT_CONSTANT((gint) ARV_GC_ERROR_OUT_OF_RANGE);
    PRINT_CONSTANT((gint) ARV_GC_ERROR_PROPERTY_NOT_DEFINED);
    PRINT_CONSTANT((gint) ARV_GC_ERROR_PVALUE_NOT_DEFINED);
    PRINT_CONSTANT((gint) ARV_GC_ERROR_READ_ONLY);
    PRINT_CONSTANT((gint) ARV_GC_ERROR_SET_FROM_STRING_UNDEFINED);
    PRINT_CONSTANT((gint) ARV_GC_IS_LINEAR_NO);
    PRINT_CONSTANT((gint) ARV_GC_IS_LINEAR_UNDEFINED);
    PRINT_CONSTANT((gint) ARV_GC_IS_LINEAR_YES);
    PRINT_CONSTANT((gint) ARV_GC_NAME_SPACE_CUSTOM);
    PRINT_CONSTANT((gint) ARV_GC_NAME_SPACE_STANDARD);
    PRINT_CONSTANT((gint) ARV_GC_NAME_SPACE_UNDEFINED);
    PRINT_CONSTANT((gint) ARV_GC_PROPERTY_NODE_TYPE_ACCESS_MODE);
    PRINT_CONSTANT((gint) ARV_GC_PROPERTY_NODE_TYPE_ADDRESS);
    PRINT_CONSTANT((gint) ARV_GC_PROPERTY_NODE_TYPE_BIT);
    PRINT_CONSTANT((gint) ARV_GC_PROPERTY_NODE_TYPE_CACHABLE);
    PRINT_CONSTANT((gint) ARV_GC_PROPERTY_NODE_TYPE_CHUNK_ID);
    PRINT_CONSTANT((gint) ARV_GC_PROPERTY_NODE_TYPE_COMMAND_VALUE);
    PRINT_CONSTANT((gint) ARV_GC_PROPERTY_NODE_TYPE_CONSTANT);
    PRINT_CONSTANT((gint) ARV_GC_PROPERTY_NODE_TYPE_DESCRIPTION);
    PRINT_CONSTANT((gint) ARV_GC_PROPERTY_NODE_TYPE_DISPLAY_NAME);
    PRINT_CONSTANT((gint) ARV_GC_PROPERTY_NODE_TYPE_DISPLAY_NOTATION);
    PRINT_CONSTANT((gint) ARV_GC_PROPERTY_NODE_TYPE_DISPLAY_PRECISION);
    PRINT_CONSTANT((gint) ARV_GC_PROPERTY_NODE_TYPE_ENDIANNESS);
    PRINT_CONSTANT((gint) ARV_GC_PROPERTY_NODE_TYPE_EVENT_ID);
    PRINT_CONSTANT((gint) ARV_GC_PROPERTY_NODE_TYPE_EXPRESSION);
    PRINT_CONSTANT((gint) ARV_GC_PROPERTY_NODE_TYPE_FORMULA);
    PRINT_CONSTANT((gint) ARV_GC_PROPERTY_NODE_TYPE_FORMULA_FROM);
    PRINT_CONSTANT((gint) ARV_GC_PROPERTY_NODE_TYPE_FORMULA_TO);
    PRINT_CONSTANT((gint) ARV_GC_PROPERTY_NODE_TYPE_IMPOSED_ACCESS_MODE);
    PRINT_CONSTANT((gint) ARV_GC_PROPERTY_NODE_TYPE_INCREMENT);
    PRINT_CONSTANT((gint) ARV_GC_PROPERTY_NODE_TYPE_IS_LINEAR);
    PRINT_CONSTANT((gint) ARV_GC_PROPERTY_NODE_TYPE_LENGTH);
    PRINT_CONSTANT((gint) ARV_GC_PROPERTY_NODE_TYPE_LSB);
    PRINT_CONSTANT((gint) ARV_GC_PROPERTY_NODE_TYPE_MAXIMUM);
    PRINT_CONSTANT((gint) ARV_GC_PROPERTY_NODE_TYPE_MINIMUM);
    PRINT_CONSTANT((gint) ARV_GC_PROPERTY_NODE_TYPE_MSB);
    PRINT_CONSTANT((gint) ARV_GC_PROPERTY_NODE_TYPE_OFF_VALUE);
    PRINT_CONSTANT((gint) ARV_GC_PROPERTY_NODE_TYPE_ON_VALUE);
    PRINT_CONSTANT((gint) ARV_GC_PROPERTY_NODE_TYPE_POLLING_TIME);
    PRINT_CONSTANT((gint) ARV_GC_PROPERTY_NODE_TYPE_P_ADDRESS);
    PRINT_CONSTANT((gint) ARV_GC_PROPERTY_NODE_TYPE_P_COMMAND_VALUE);
    PRINT_CONSTANT((gint) ARV_GC_PROPERTY_NODE_TYPE_P_FEATURE);
    PRINT_CONSTANT((gint) ARV_GC_PROPERTY_NODE_TYPE_P_INCREMENT);
    PRINT_CONSTANT((gint) ARV_GC_PROPERTY_NODE_TYPE_P_INDEX);
    PRINT_CONSTANT((gint) ARV_GC_PROPERTY_NODE_TYPE_P_INVALIDATOR);
    PRINT_CONSTANT((gint) ARV_GC_PROPERTY_NODE_TYPE_P_IS_AVAILABLE);
    PRINT_CONSTANT((gint) ARV_GC_PROPERTY_NODE_TYPE_P_IS_IMPLEMENTED);
    PRINT_CONSTANT((gint) ARV_GC_PROPERTY_NODE_TYPE_P_IS_LOCKED);
    PRINT_CONSTANT((gint) ARV_GC_PROPERTY_NODE_TYPE_P_LENGTH);
    PRINT_CONSTANT((gint) ARV_GC_PROPERTY_NODE_TYPE_P_MAXIMUM);
    PRINT_CONSTANT((gint) ARV_GC_PROPERTY_NODE_TYPE_P_MINIMUM);
    PRINT_CONSTANT((gint) ARV_GC_PROPERTY_NODE_TYPE_P_PORT);
    PRINT_CONSTANT((gint) ARV_GC_PROPERTY_NODE_TYPE_P_SELECTED);
    PRINT_CONSTANT((gint) ARV_GC_PROPERTY_NODE_TYPE_P_UNKNONW);
    PRINT_CONSTANT((gint) ARV_GC_PROPERTY_NODE_TYPE_P_VALUE);
    PRINT_CONSTANT((gint) ARV_GC_PROPERTY_NODE_TYPE_P_VALUE_DEFAULT);
    PRINT_CONSTANT((gint) ARV_GC_PROPERTY_NODE_TYPE_P_VALUE_INDEXED);
    PRINT_CONSTANT((gint) ARV_GC_PROPERTY_NODE_TYPE_P_VARIABLE);
    PRINT_CONSTANT((gint) ARV_GC_PROPERTY_NODE_TYPE_REPRESENTATION);
    PRINT_CONSTANT((gint) ARV_GC_PROPERTY_NODE_TYPE_SIGN);
    PRINT_CONSTANT((gint) ARV_GC_PROPERTY_NODE_TYPE_SLOPE);
    PRINT_CONSTANT((gint) ARV_GC_PROPERTY_NODE_TYPE_STREAMABLE);
    PRINT_CONSTANT((gint) ARV_GC_PROPERTY_NODE_TYPE_TOOLTIP);
    PRINT_CONSTANT((gint) ARV_GC_PROPERTY_NODE_TYPE_UNIT);
    PRINT_CONSTANT((gint) ARV_GC_PROPERTY_NODE_TYPE_UNKNOWN);
    PRINT_CONSTANT((gint) ARV_GC_PROPERTY_NODE_TYPE_VALUE);
    PRINT_CONSTANT((gint) ARV_GC_PROPERTY_NODE_TYPE_VALUE_DEFAULT);
    PRINT_CONSTANT((gint) ARV_GC_PROPERTY_NODE_TYPE_VALUE_INDEXED);
    PRINT_CONSTANT((gint) ARV_GC_PROPERTY_NODE_TYPE_VISIBILITY);
    PRINT_CONSTANT((gint) ARV_GC_REPRESENTATION_BOOLEAN);
    PRINT_CONSTANT((gint) ARV_GC_REPRESENTATION_HEX_NUMBER);
    PRINT_CONSTANT((gint) ARV_GC_REPRESENTATION_IPV4_ADDRESS);
    PRINT_CONSTANT((gint) ARV_GC_REPRESENTATION_LINEAR);
    PRINT_CONSTANT((gint) ARV_GC_REPRESENTATION_LOGARITHMIC);
    PRINT_CONSTANT((gint) ARV_GC_REPRESENTATION_MAC_ADDRESS);
    PRINT_CONSTANT((gint) ARV_GC_REPRESENTATION_PURE_NUMBER);
    PRINT_CONSTANT((gint) ARV_GC_REPRESENTATION_UNDEFINED);
    PRINT_CONSTANT((gint) ARV_GC_SIGNEDNESS_SIGNED);
    PRINT_CONSTANT((gint) ARV_GC_SIGNEDNESS_UNDEFINED);
    PRINT_CONSTANT((gint) ARV_GC_SIGNEDNESS_UNSIGNED);
    PRINT_CONSTANT((gint) ARV_GC_STREAMABLE_NO);
    PRINT_CONSTANT((gint) ARV_GC_STREAMABLE_UNDEFINED);
    PRINT_CONSTANT((gint) ARV_GC_STREAMABLE_YES);
    PRINT_CONSTANT((gint) ARV_GC_VISIBILITY_BEGINNER);
    PRINT_CONSTANT((gint) ARV_GC_VISIBILITY_EXPERT);
    PRINT_CONSTANT((gint) ARV_GC_VISIBILITY_GURU);
    PRINT_CONSTANT((gint) ARV_GC_VISIBILITY_INVISIBLE);
    PRINT_CONSTANT((gint) ARV_GC_VISIBILITY_UNDEFINED);
    PRINT_CONSTANT(ARV_GV_FAKE_CAMERA_DEFAULT_INTERFACE);
    PRINT_CONSTANT(ARV_GV_FAKE_CAMERA_DEFAULT_SERIAL_NUMBER);
    PRINT_CONSTANT((gint) ARV_GV_PACKET_SIZE_ADJUSTMENT_ALWAYS);
    PRINT_CONSTANT((gint) ARV_GV_PACKET_SIZE_ADJUSTMENT_DEFAULT);
    PRINT_CONSTANT((gint) ARV_GV_PACKET_SIZE_ADJUSTMENT_NEVER);
    PRINT_CONSTANT((gint) ARV_GV_PACKET_SIZE_ADJUSTMENT_ONCE);
    PRINT_CONSTANT((gint) ARV_GV_PACKET_SIZE_ADJUSTMENT_ON_FAILURE);
    PRINT_CONSTANT((gint) ARV_GV_PACKET_SIZE_ADJUSTMENT_ON_FAILURE_ONCE);
    PRINT_CONSTANT((gint) ARV_GV_STREAM_OPTION_NONE);
    PRINT_CONSTANT((gint) ARV_GV_STREAM_OPTION_PACKET_SOCKET_DISABLED);
    PRINT_CONSTANT((gint) ARV_GV_STREAM_PACKET_RESEND_ALWAYS);
    PRINT_CONSTANT((gint) ARV_GV_STREAM_PACKET_RESEND_NEVER);
    PRINT_CONSTANT((gint) ARV_GV_STREAM_SOCKET_BUFFER_AUTO);
    PRINT_CONSTANT((gint) ARV_GV_STREAM_SOCKET_BUFFER_FIXED);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_BAYER_BG_10);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_BAYER_BG_10P);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_BAYER_BG_10_PACKED);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_BAYER_BG_12);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_BAYER_BG_12P);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_BAYER_BG_12_PACKED);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_BAYER_BG_16);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_BAYER_BG_8);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_BAYER_GB_10);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_BAYER_GB_10P);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_BAYER_GB_10_PACKED);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_BAYER_GB_12);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_BAYER_GB_12P);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_BAYER_GB_12_PACKED);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_BAYER_GB_16);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_BAYER_GB_8);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_BAYER_GR_10);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_BAYER_GR_10P);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_BAYER_GR_10_PACKED);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_BAYER_GR_12);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_BAYER_GR_12P);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_BAYER_GR_12_PACKED);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_BAYER_GR_16);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_BAYER_GR_8);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_BAYER_RG_10);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_BAYER_RG_10P);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_BAYER_RG_10_PACKED);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_BAYER_RG_12);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_BAYER_RG_12P);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_BAYER_RG_12_PACKED);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_BAYER_RG_16);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_BAYER_RG_8);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_BGRA_8_PACKED);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_BGR_10_PACKED);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_BGR_12_PACKED);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_BGR_8_PACKED);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_CUSTOM_BAYER_BG_12_PACKED);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_CUSTOM_BAYER_BG_16);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_CUSTOM_BAYER_GB_12_PACKED);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_CUSTOM_BAYER_GB_16);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_CUSTOM_BAYER_GR_12_PACKED);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_CUSTOM_BAYER_GR_16);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_CUSTOM_BAYER_RG_12_PACKED);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_CUSTOM_BAYER_RG_16);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_CUSTOM_YUV_422_YUYV_PACKED);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_MONO_10);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_MONO_10_PACKED);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_MONO_12);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_MONO_12_PACKED);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_MONO_14);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_MONO_16);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_MONO_8);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_MONO_8_SIGNED);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_RGBA_8_PACKED);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_RGB_10_PACKED);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_RGB_10_PLANAR);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_RGB_12_PACKED);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_RGB_12_PLANAR);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_RGB_16_PLANAR);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_RGB_8_PACKED);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_RGB_8_PLANAR);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_YUV_411_PACKED);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_YUV_422_PACKED);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_YUV_422_YUYV_PACKED);
    PRINT_CONSTANT(ARV_PIXEL_FORMAT_YUV_444_PACKED);
    PRINT_CONSTANT((gint) ARV_RANGE_CHECK_POLICY_DEBUG);
    PRINT_CONSTANT((gint) ARV_RANGE_CHECK_POLICY_DEFAULT);
    PRINT_CONSTANT((gint) ARV_RANGE_CHECK_POLICY_DISABLE);
    PRINT_CONSTANT((gint) ARV_RANGE_CHECK_POLICY_ENABLE);
    PRINT_CONSTANT((gint) ARV_REGISTER_CACHE_POLICY_DEBUG);
    PRINT_CONSTANT((gint) ARV_REGISTER_CACHE_POLICY_DEFAULT);
    PRINT_CONSTANT((gint) ARV_REGISTER_CACHE_POLICY_DISABLE);
    PRINT_CONSTANT((gint) ARV_REGISTER_CACHE_POLICY_ENABLE);
    PRINT_CONSTANT((gint) ARV_STREAM_CALLBACK_TYPE_BUFFER_DONE);
    PRINT_CONSTANT((gint) ARV_STREAM_CALLBACK_TYPE_EXIT);
    PRINT_CONSTANT((gint) ARV_STREAM_CALLBACK_TYPE_INIT);
    PRINT_CONSTANT((gint) ARV_STREAM_CALLBACK_TYPE_START_BUFFER);
    PRINT_CONSTANT((gint) ARV_XML_SCHEMA_ERROR_INVALID_STRUCTURE);
    return 0;
}
