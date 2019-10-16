<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Address_NE_AI_ACCESO_PADRELUIS</name>
   <tag></tag>
   <elementGuidId>4b80f79c-7a39-417e-83a8-a61cae9ce549</elementGuidId>
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
               &lt;level4>MAR DEL PLATA-BS. AS.&lt;/level4>
               &lt;!--Optional:-->
               &lt;level5>&lt;/level5>
               &lt;!--Optional:-->
               &lt;street>ACCESO PADRE LUIS VARETTO KM 3200 N&lt;/street>
               &lt;!--Optional:-->
               &lt;houseNumber>&lt;/houseNumber>
               &lt;!--Optional:-->
               &lt;floor>&lt;/floor>
               &lt;!--Optional:-->
               &lt;unit>&lt;/unit>
               &lt;!--Optional:-->
               &lt;postalCode>7601&lt;/postalCode>
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




/*Verificacion Test Case*/

WS.verifyElementText(response, 'addressNormalize2Response.return.status', 'NE')
WS.verifyElementText(response, 'addressNormalize2Response.return.statusReason', 'AI')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.geoType', '5')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level1', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level2', 'BUENOS AIRES')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level3', 'GENERAL PUEYRREDON')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level4', 'SIERRA DE LOS PADRES')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level5', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.street', 'PADRE LUIS VARETTO')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.houseNumber', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.floor', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.unit', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.latitude', '-37.939010')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.longitude', '-57.772510')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.postalCode', '7601')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.additionalPostalCode', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.fromStreetNumber', '1')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.toStreetNumber', '2')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.betweenStreet1', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.betweenStreet2', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.additionalData', 'KM 3200  N')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.corner', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.placeType', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.place', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.placeReference', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.numberAlternativeAddresses', '0')
assertThat(response.getResponseText()).contains('&lt;name>latitudLocalidad&lt;/name>&lt;value>-37.949173&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>cpa&lt;/name>&lt;value>&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>level4Longitude&lt;/name>&lt;value>-57.777804&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>longitudLocalidad&lt;/name>&lt;value>-57.777804&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>puerta&lt;/name>&lt;value>NO RELEVADO&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>NISE&lt;/name>&lt;value>0&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>level4Latitude&lt;/name>&lt;value>-37.949173&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>zonaRiesgo&lt;/name>&lt;value>N&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>merlinRiskArea&lt;/name>&lt;value>N&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>postalCertifiedAddresses&lt;/name>&lt;value>NO RELEVADO&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>nise&lt;/name>&lt;value>0&lt;/value>')



/** Verificacion WS unitaria


WS.verifyElementText(response, 'addressNormalize2Response.return.status', 'NE')
WS.verifyElementText(response, 'addressNormalize2Response.return.statusReason', 'AI')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.geoType', '5')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level1', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level2', 'BUENOS AIRES')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level3', 'GENERAL PUEYRREDON')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level4', 'SIERRA DE LOS PADRES')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level5', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.street', 'PADRE LUIS VARETTO')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.houseNumber', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.floor', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.unit', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.latitude', '-37.939010')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.longitude', '-57.772510')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.postalCode', '7601')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.additionalPostalCode', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.fromStreetNumber', '1')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.toStreetNumber', '2')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.betweenStreet1', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.betweenStreet2', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.additionalData', 'KM 3200  N')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.corner', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.placeType', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.place', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.placeReference', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.numberAlternativeAddresses', '0')
assertThat(response.getResponseText()).contains('&lt;name>latitudLocalidad&lt;/name>&lt;value>-37.949173&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>cpa&lt;/name>&lt;value/>')
assertThat(response.getResponseText()).contains('&lt;name>level4Longitude&lt;/name>&lt;value>-57.777804&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>longitudLocalidad&lt;/name>&lt;value>-57.777804&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>puerta&lt;/name>&lt;value>NO RELEVADO&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>NISE&lt;/name>&lt;value>0&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>level4Latitude&lt;/name>&lt;value>-37.949173&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>zonaRiesgo&lt;/name>&lt;value>N&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>merlinRiskArea&lt;/name>&lt;value>N&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>postalCertifiedAddresses&lt;/name>&lt;value>NO RELEVADO&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>nise&lt;/name>&lt;value>0&lt;/value>')
**/</verificationScript>
   <wsdlAddress>${Address_ARG2}</wsdlAddress>
</WebServiceRequestEntity>
