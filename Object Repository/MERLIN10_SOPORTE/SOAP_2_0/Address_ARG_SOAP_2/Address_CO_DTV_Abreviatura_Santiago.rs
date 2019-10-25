<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Address_CO_DTV_Abreviatura_Santiago</name>
   <tag></tag>
   <elementGuidId>1c6d855c-de13-42c6-ba8d-5a5911b9083f</elementGuidId>
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
            &lt;clientAccessCode>59078f94b7c6bb08bf072449be86c2e0&lt;/clientAccessCode>
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
               &lt;level2>Santiago del estero&lt;/level2>
               &lt;!--Optional:-->
               &lt;level3>&lt;/level3>
               &lt;!--Optional:-->
               &lt;level4>Santiago del estero&lt;/level4>
               &lt;!--Optional:-->
               &lt;level5>&lt;/level5>
               &lt;!--Optional:-->
               &lt;street>Av aguirre 705&lt;/street>
               &lt;!--Optional:-->
               &lt;houseNumber> &lt;/houseNumber>
               &lt;!--Optional:-->
               &lt;floor>&lt;/floor>
               &lt;!--Optional:-->
               &lt;unit>&lt;/unit>
               &lt;!--Optional:-->
               &lt;postalCode>&lt;/postalCode>
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
      <id>4d59abac-efa3-41b7-bd77-7640915b76d1</id>
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


/**Verificacion Test Case**/

WS.verifyElementText(response, 'addressNormalize2Response.return.status', 'CO')
WS.verifyElementText(response, 'addressNormalize2Response.return.statusReason', 'SM')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.geoType', '1')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level1', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level2', 'SGO DEL ESTERO')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level3', 'JUAN FELIPE BORGES')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level4', 'SANTIAGO DEL ESTERO')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level5', 'SAN MARTIN')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.street', 'AVDA AGUIRRE')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.houseNumber', '705')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.floor', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.unit', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.latitude', '-27.803682')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.longitude', '-64.271649')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.postalCode', '4200')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.additionalPostalCode', 'G4200HWH')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.fromStreetNumber', '1')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.toStreetNumber', '3100')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.betweenStreet1', 'GAUCHO ANTONIO RIVERO')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.betweenStreet2', 'YAPEYU')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.corner', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.placeType', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.place', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.placeReference', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.numberAlternativeAddresses', '0')
assertThat(response.getResponseText()).contains('&lt;name>level4longitudeDTV&lt;/name>&lt;value>-64.262336&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>level4Longitude&lt;/name>&lt;value>-64.262336&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>level4Latitude&lt;/name>&lt;value>-27.793186&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>maqStatus&lt;/name>&lt;value>NO&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>maqValue&lt;/name>&lt;value>&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>maqConcept&lt;/name>&lt;value>&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>nise&lt;/name>&lt;value>3&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>merlinRiskArea&lt;/name>&lt;value>N&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>level4latitudeDTV&lt;/name>&lt;value>-27.793186&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>postalCertifiedAddresses&lt;/name>&lt;value>NO RELEVADO&lt;/value>')


/**Verificacion WS unitario

WS.verifyElementText(response, 'addressNormalize2Response.return.status', 'CO')
WS.verifyElementText(response, 'addressNormalize2Response.return.statusReason', 'SM')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.geoType', '1')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level1', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level2', 'SGO DEL ESTERO')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level3', 'JUAN FELIPE BORGES')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level4', 'SANTIAGO DEL ESTERO')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level5', 'SAN MARTIN')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.street', 'AVDA AGUIRRE')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.houseNumber', '705')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.floor', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.unit', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.latitude', '-27.803682')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.longitude', '-64.271649')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.postalCode', '4200')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.additionalPostalCode', 'G4200HWH')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.fromStreetNumber', '1')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.toStreetNumber', '3100')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.betweenStreet1', 'GAUCHO ANTONIO RIVERO')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.betweenStreet2', 'YAPEYU')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.corner', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.placeType', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.place', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.placeReference', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.numberAlternativeAddresses', '0')
assertThat(response.getResponseText()).contains('&lt;name>level4longitudeDTV&lt;/name>&lt;value>-64.262336&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>level4Longitude&lt;/name>&lt;value>-64.262336&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>level4Latitude&lt;/name>&lt;value>-27.793186&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>maqStatus&lt;/name>&lt;value>NO&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>maqValue&lt;/name>&lt;value/>')
assertThat(response.getResponseText()).contains('&lt;name>maqConcept&lt;/name>&lt;value/>')
assertThat(response.getResponseText()).contains('&lt;name>nise&lt;/name>&lt;value>3&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>merlinRiskArea&lt;/name>&lt;value>N&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>level4latitudeDTV&lt;/name>&lt;value>-27.793186&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>postalCertifiedAddresses&lt;/name>&lt;value>NO RELEVADO&lt;/value>')

**/</verificationScript>
   <wsdlAddress>${Address_ARG2}</wsdlAddress>
</WebServiceRequestEntity>
