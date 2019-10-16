<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Address_CO_Santander_Abreviatura2</name>
   <tag></tag>
   <elementGuidId>b477e0aa-ed90-488d-821c-ddcbd7bcbe49</elementGuidId>
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
            &lt;clientAccessCode>58e689da7b9bb2cbef011e4d795653da&lt;/clientAccessCode>
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
               &lt;level2>&lt;/level2>
               &lt;!--Optional:-->
               &lt;level3>&lt;/level3>
               &lt;!--Optional:-->
               &lt;level4>ROSARIO&lt;/level4>
               &lt;!--Optional:-->
               &lt;level5>&lt;/level5>
               &lt;!--Optional:-->
               &lt;street>10 DE DICIEMBRE 901&lt;/street>
               &lt;!--Optional:-->
               &lt;houseNumber>&lt;/houseNumber>
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

/*verificacion testcases*/
WS.verifyElementText(response, 'addressNormalize2Response.return.status', 'CO')
WS.verifyElementText(response, 'addressNormalize2Response.return.statusReason', 'SM')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.geoType', '1')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level1', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level2', 'SANTA FE')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level3', 'ROSARIO')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level4', 'ROSARIO')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level5', 'REMEDIOS DE ESCALADA DE SAN MARTIN')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.streetType', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.street', '10 DE DICIEMBRE')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.houseNumber', '901')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.floor', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.unit', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.latitude', '-32.942069')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.longitude', '-60.676428')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.postalCode', '2002')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.additionalPostalCode', 'S2002SOA')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.fromStreetNumber', '901')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.toStreetNumber', '1000')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.additionalData', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.betweenStreet1', 'SAN LUIS')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.betweenStreet2', 'RIOJA')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.corner', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.placeType', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.place', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.placeReference', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.numberAlternativeAddresses', '0')
assertThat(response.getResponseText()).contains('&lt;name>nise&lt;/name>&lt;value>5&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>maqStatus&lt;/name>&lt;value>NO&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>maqValue&lt;/name>&lt;value>&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>maqConcept&lt;/name>&lt;value>&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>level4Latitude&lt;/name>&lt;value>-32.955935&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>merlinRiskArea&lt;/name>&lt;value>N&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>postalCertifiedAddresses&lt;/name>&lt;value>NO RELEVADO&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>level4Longitude&lt;/name>&lt;value>-60.651352&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>level4abbreviated12&lt;/name>&lt;value>ROSARIO&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>level5abbreviated30&lt;/name>&lt;value>R DE ESCALADA SAN MARTIN&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>abbreviatedStreet20&lt;/name>&lt;value>10 DE DICIEMBRE&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>level4abbreviated30&lt;/name>&lt;value>ROSARIO&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>abbreviatedStreet30&lt;/name>&lt;value>10 DE DICIEMBRE&lt;/value>')

/**verificacion WS unitaria
WS.verifyElementText(response, 'addressNormalize2Response.return.status', 'CO')
WS.verifyElementText(response, 'addressNormalize2Response.return.statusReason', 'SM')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.geoType', '1')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level1', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level2', 'SANTA FE')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level3', 'ROSARIO')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level4', 'ROSARIO')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level5', 'REMEDIOS DE ESCALADA DE SAN MARTIN')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.streetType', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.street', '10 DE DICIEMBRE')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.houseNumber', '901')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.floor', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.unit', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.latitude', '-32.942069')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.longitude', '-60.676428')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.postalCode', '2002')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.additionalPostalCode', 'S2002SOA')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.fromStreetNumber', '901')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.toStreetNumber', '1000')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.additionalData', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.betweenStreet1', 'SAN LUIS')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.betweenStreet2', 'RIOJA')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.corner', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.placeType', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.place', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.placeReference', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.numberAlternativeAddresses', '0')
assertThat(response.getResponseText()).contains('&lt;name>nise&lt;/name>&lt;value>5&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>maqStatus&lt;/name>&lt;value>NO&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>maqValue&lt;/name>&lt;value/>')
assertThat(response.getResponseText()).contains('&lt;name>maqConcept&lt;/name>&lt;value/>')
assertThat(response.getResponseText()).contains('&lt;name>level4Latitude&lt;/name>&lt;value>-32.955935&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>merlinRiskArea&lt;/name>&lt;value>N&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>postalCertifiedAddresses&lt;/name>&lt;value>NO RELEVADO&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>level4Longitude&lt;/name>&lt;value>-60.651352&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>level4abbreviated12&lt;/name>&lt;value>ROSARIO&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>level5abbreviated30&lt;/name>&lt;value>R DE ESCALADA SAN MARTIN&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>abbreviatedStreet20&lt;/name>&lt;value>10 DE DICIEMBRE&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>level4abbreviated30&lt;/name>&lt;value>ROSARIO&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>abbreviatedStreet30&lt;/name>&lt;value>10 DE DICIEMBRE&lt;/value>')
**/
</verificationScript>
   <wsdlAddress>${Address_ARG2}</wsdlAddress>
</WebServiceRequestEntity>
