<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Address_NE_AI_DTV_cp</name>
   <tag></tag>
   <elementGuidId>639c5061-fc6f-4f15-82f8-10ae782caa8d</elementGuidId>
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
            &lt;clientAccessCode>9b53cf1ab4e1eb748b28e150fdc7c638&lt;/clientAccessCode>
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
               &lt;level2>MENDOZA&lt;/level2>
               &lt;!--Optional:-->
               &lt;level3>&lt;/level3>
               &lt;!--Optional:-->
               &lt;level4>VILLA NUEVA DE GUAYMALLEN&lt;/level4>
               &lt;!--Optional:-->
               &lt;level5>&lt;/level5>
               &lt;!--Optional:-->
               &lt;street>ALPATACAL 1489&lt;/street>
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




/*Verificacion Test Case*/

WS.verifyElementText(response, 'addressNormalize2Response.return.status', 'NE')
WS.verifyElementText(response, 'addressNormalize2Response.return.statusReason', 'AI')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.geoType', '3')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level1', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level2', 'MENDOZA')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level3', 'GUAYMALLEN')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level4', 'V NUEVA DE GUAYMALLEN')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level5', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.street', 'ALPATACAL')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.houseNumber', '1489')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.floor', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.unit', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.latitude', '-32.894984')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.longitude', '-68.780130')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.postalCode', '5521')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.additionalPostalCode', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.fromStreetNumber', '301')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.toStreetNumber', '1450')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.betweenStreet1', 'DR TOMAS GODOY CRUZ')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.betweenStreet2', 'GRAL MANUEL BELGRANO')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.corner', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.placeType', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.place', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.placeReference', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.numberAlternativeAddresses', '0')
assertThat(response.getResponseText()).contains('&lt;name>latitudLocalidad&lt;/name>&lt;value>-32.906910&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>cpa&lt;/name>&lt;value>&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>level4Longitude&lt;/name>&lt;value>-68.791556&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>longitudLocalidad&lt;/name>&lt;value>-68.791556&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>puerta&lt;/name>&lt;value>NO RELEVADO&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>NISE&lt;/name>&lt;value>2&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>level4Latitude&lt;/name>&lt;value>-32.906910&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>zonaRiesgo&lt;/name>&lt;value>N&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>merlinRiskArea&lt;/name>&lt;value>N&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>postalCertifiedAddresses&lt;/name>&lt;value>NO RELEVADO&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>level4longitudeDTV&lt;/name>&lt;value>-68.791556&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>level4latitudeDTV&lt;/name>&lt;value>-32.906910&lt;/value>')



/** Verificacion WS unitaria

WS.verifyElementText(response, 'addressNormalize2Response.return.status', 'NE')
WS.verifyElementText(response, 'addressNormalize2Response.return.statusReason', 'AI')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.geoType', '3')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level1', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level2', 'MENDOZA')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level3', 'GUAYMALLEN')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level4', 'V NUEVA DE GUAYMALLEN')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level5', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.street', 'ALPATACAL')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.houseNumber', '1489')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.floor', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.unit', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.latitude', '-32.894984')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.longitude', '-68.780130')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.postalCode', '5521')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.additionalPostalCode', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.fromStreetNumber', '301')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.toStreetNumber', '1450')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.betweenStreet1', 'DR TOMAS GODOY CRUZ')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.betweenStreet2', 'GRAL MANUEL BELGRANO')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.corner', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.placeType', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.place', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.placeReference', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.numberAlternativeAddresses', '0')
assertThat(response.getResponseText()).contains('&lt;name>latitudLocalidad&lt;/name>&lt;value>-32.906910&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>cpa&lt;/name>&lt;value/>')
assertThat(response.getResponseText()).contains('&lt;name>level4Longitude&lt;/name>&lt;value>-68.791556&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>longitudLocalidad&lt;/name>&lt;value>-68.791556&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>puerta&lt;/name>&lt;value>NO RELEVADO&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>NISE&lt;/name>&lt;value>2&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>level4Latitude&lt;/name>&lt;value>-32.906910&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>zonaRiesgo&lt;/name>&lt;value>N&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>merlinRiskArea&lt;/name>&lt;value>N&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>postalCertifiedAddresses&lt;/name>&lt;value>NO RELEVADO&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>level4longitudeDTV&lt;/name>&lt;value>-68.791556&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>level4latitudeDTV&lt;/name>&lt;value>-32.906910&lt;/value>')
**/</verificationScript>
   <wsdlAddress>${Address_ARG2}</wsdlAddress>
</WebServiceRequestEntity>
