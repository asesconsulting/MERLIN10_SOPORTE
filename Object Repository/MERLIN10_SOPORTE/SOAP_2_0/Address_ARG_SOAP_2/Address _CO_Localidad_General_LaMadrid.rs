<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Address _CO_Localidad_General_LaMadrid</name>
   <tag></tag>
   <elementGuidId>3d4df98f-619f-4427-8b3b-2db1c004bec6</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:soap=&quot;http://soap2.addressonline.ases.com/&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;soap:addressNormalize2>
         &lt;!--Optional:-->
         &lt;addressNormalize>
            &lt;!--Optional:-->
            &lt;clientAccessCode>aea243aba41084aa098b3a70eeb63ddf&lt;/clientAccessCode>
            &lt;!--Optional:-->
            &lt;customAdaptersKeys>
               &lt;!--Zero or more repetitions:-->
               &lt;customAdapter>&lt;/customAdapter>
            &lt;/customAdaptersKeys>
            &lt;!--Optional:-->
            &lt;xAddress>
               &lt;!--Optional:-->
               &lt;level1>&lt;/level1>
               &lt;!--Optional:-->
               &lt;level2>BUENOS AIRES&lt;/level2>
               &lt;!--Optional:-->
               &lt;level3>&lt;/level3>
               &lt;!--Optional:-->
               &lt;level4>GENERAL LA MADRID&lt;/level4>
               &lt;!--Optional:-->
               &lt;level5>&lt;/level5>
               &lt;!--Optional:-->
               &lt;street>AZCUENAGA&lt;/street>
               &lt;!--Optional:-->
               &lt;houseNumber>540&lt;/houseNumber>
               &lt;!--Optional:-->
               &lt;floor>&lt;/floor>
               &lt;!--Optional:-->
               &lt;unit>&lt;/unit>
               &lt;!--Optional:-->
               &lt;postalCode>7406&lt;/postalCode>
               &lt;!--Optional:-->
               &lt;additionalData>&lt;/additionalData>
               &lt;!--Optional:-->
               &lt;betweenStreet1>&lt;/betweenStreet1>
               &lt;!--Optional:-->
               &lt;betweenStreet2>&lt;/betweenStreet2>
            &lt;/xAddress>
         &lt;/addressNormalize>
      &lt;/soap:addressNormalize2>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>addressNormalize2</soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.Address_ARG2</defaultValue>
      <description></description>
      <id>1398d2bc-b06a-4075-891a-de3127067976</id>
      <masked>false</masked>
      <name>Address_ARG2</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

/**Validacion test case**/

WS.verifyElementText(response, 'addressNormalize2Response.return.status', 'CO')
WS.verifyElementText(response, 'addressNormalize2Response.return.statusReason', 'SM')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.geoType', '1')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level1', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level2', 'BUENOS AIRES')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level3', 'GENERAL LA MADRID')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level4', 'GENERAL LA MADRID')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level5', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.streetType', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.street', 'AZCUENAGA')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.houseNumber', '540')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.floor', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.unit', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.latitude', '-37.250882')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.longitude', '-61.263749')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.postalCode', '7406')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.additionalPostalCode', 'B7406AEL')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.fromStreetNumber', '1')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.toStreetNumber', '1700')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.additionalData', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.betweenStreet1', '25 DE MAYO')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.betweenStreet2', 'DOMINGO FAUSTINO SARMIENTO')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.corner', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.placeType', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.place', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.placeReference', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.numberAlternativeAddresses', '0')
assertThat(response.getResponseText()).contains('&lt;name>maqConcept&lt;/name>&lt;value>&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>maqStatus&lt;/name>&lt;value>NO&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>concepto&lt;/name>&lt;value>&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>tipoVivienda&lt;/name>&lt;value>otro&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>longitudLocalidad&lt;/name>&lt;value>-61.257452&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>empresa&lt;/name>&lt;value>NO&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>nivelRiesgo&lt;/name>&lt;value>&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>nise&lt;/name>&lt;value>0&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>cpa&lt;/name>&lt;value>B7406AEL&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>level4Latitude&lt;/name>&lt;value>-37.247832&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>postalCertifiedAddresses&lt;/name>&lt;value>SI&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>inMaq&lt;/name>&lt;value>0&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>puerta&lt;/name>&lt;value>SI&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>level4Longitude&lt;/name>&lt;value>-61.257452&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>merlinRiskArea&lt;/name>&lt;value>N&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>NISE&lt;/name>&lt;value>0&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>zonaRiesgo&lt;/name>&lt;value>N&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>latitudLocalidad&lt;/name>&lt;value>-37.247832&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>maqValue&lt;/name>&lt;value>&lt;/value>')


/**Validacion WS Unitaria

WS.verifyElementText(response, 'addressNormalize2Response.return.status', 'CO')
WS.verifyElementText(response, 'addressNormalize2Response.return.statusReason', 'SM')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.geoType', '1')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level1', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level2', 'BUENOS AIRES')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level3', 'GENERAL LA MADRID')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level4', 'GENERAL LA MADRID')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level5', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.streetType', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.street', 'AZCUENAGA')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.houseNumber', '540')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.floor', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.unit', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.latitude', '-37.250882')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.longitude', '-61.263749')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.postalCode', '7406')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.additionalPostalCode', 'B7406AEL')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.fromStreetNumber', '1')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.toStreetNumber', '1700')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.additionalData', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.betweenStreet1', '25 DE MAYO')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.betweenStreet2', 'DOMINGO FAUSTINO SARMIENTO')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.corner', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.placeType', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.place', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.placeReference', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.numberAlternativeAddresses', '0')
assertThat(response.getResponseText()).contains('&lt;name>maqConcept&lt;/name>&lt;value/>')
assertThat(response.getResponseText()).contains('&lt;name>maqStatus&lt;/name>&lt;value>NO&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>concepto&lt;/name>&lt;value/>')
assertThat(response.getResponseText()).contains('&lt;name>tipoVivienda&lt;/name>&lt;value>otro&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>longitudLocalidad&lt;/name>&lt;value>-61.257452&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>empresa&lt;/name>&lt;value>NO&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>nivelRiesgo&lt;/name>&lt;value/>')
assertThat(response.getResponseText()).contains('&lt;name>nise&lt;/name>&lt;value>0&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>cpa&lt;/name>&lt;value>B7406AEL&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>level4Latitude&lt;/name>&lt;value>-37.247832&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>postalCertifiedAddresses&lt;/name>&lt;value>SI&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>inMaq&lt;/name>&lt;value>0&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>puerta&lt;/name>&lt;value>SI&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>level4Longitude&lt;/name>&lt;value>-61.257452&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>merlinRiskArea&lt;/name>&lt;value>N&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>NISE&lt;/name>&lt;value>0&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>zonaRiesgo&lt;/name>&lt;value>N&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>latitudLocalidad&lt;/name>&lt;value>-37.247832&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>maqValue&lt;/name>&lt;value/>')
**/
</verificationScript>
   <wsdlAddress>${Address_ARG2}</wsdlAddress>
</WebServiceRequestEntity>
